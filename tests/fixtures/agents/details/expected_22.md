---
id: "test-022"
title: "Configuration Management Library"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a configuration management library for Go applications. Supports loading from files (YAML, JSON, TOML), environment variables, and CLI flags with layered precedence. Includes validation and type-safe access.

## Constraints

- Zero external dependencies for core functionality
- Support hot-reloading of config files

## Implementation Notes

### Technology Stack
- **Language:** Go 1.21+
- **File Parsing:** encoding/json (stdlib), yaml.v3, toml (optional)
- **Validation:** Custom struct tag parser

### Core Interface
```go
type Config interface {
    // Load configuration from all sources
    Load() error

    // Get typed values
    String(key string) string
    Int(key string) int
    Bool(key string) bool
    Duration(key string) time.Duration

    // Unmarshal into struct
    Unmarshal(v interface{}) error

    // Watch for changes
    OnChange(callback func())
}

type Option func(*configLoader)

func WithFile(path string) Option
func WithEnvPrefix(prefix string) Option
func WithDefaults(defaults map[string]interface{}) Option
```

### Config Struct Tags
```go
type ServerConfig struct {
    Host     string        `config:"host" env:"SERVER_HOST" default:"localhost"`
    Port     int           `config:"port" env:"SERVER_PORT" default:"8080" validate:"min=1,max=65535"`
    Timeout  time.Duration `config:"timeout" env:"SERVER_TIMEOUT" default:"30s"`
    Debug    bool          `config:"debug" env:"DEBUG" default:"false"`
    Database DatabaseConfig `config:"database"`
}

type DatabaseConfig struct {
    URL         string `config:"url" env:"DATABASE_URL" validate:"required,url"`
    MaxConns    int    `config:"max_conns" default:"10" validate:"min=1"`
    MaxIdleTime string `config:"max_idle_time" default:"5m"`
}
```

### Loader Implementation
```go
type configLoader struct {
    sources   []Source
    values    map[string]interface{}
    mu        sync.RWMutex
    callbacks []func()
    envPrefix string
}

type Source interface {
    Load() (map[string]interface{}, error)
    Watch(callback func()) error
}

func New(opts ...Option) *configLoader {
    c := &configLoader{
        values: make(map[string]interface{}),
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}

func (c *configLoader) Load() error {
    c.mu.Lock()
    defer c.mu.Unlock()

    // Clear existing values
    c.values = make(map[string]interface{})

    // Load sources in order (later sources override earlier)
    for _, source := range c.sources {
        vals, err := source.Load()
        if err != nil {
            return fmt.Errorf("load source: %w", err)
        }
        c.merge(vals)
    }

    // Load environment variables (highest precedence)
    c.loadEnv()

    return nil
}
```

### Environment Variable Loading
```go
func (c *configLoader) loadEnv() {
    for _, env := range os.Environ() {
        parts := strings.SplitN(env, "=", 2)
        if len(parts) != 2 {
            continue
        }
        key, value := parts[0], parts[1]

        if c.envPrefix != "" && !strings.HasPrefix(key, c.envPrefix) {
            continue
        }

        // Convert ENV_VAR_NAME to env.var.name
        configKey := strings.ToLower(strings.TrimPrefix(key, c.envPrefix+"_"))
        configKey = strings.ReplaceAll(configKey, "_", ".")

        c.values[configKey] = value
    }
}
```

### Unmarshal with Validation
```go
func (c *configLoader) Unmarshal(v interface{}) error {
    c.mu.RLock()
    defer c.mu.RUnlock()

    rv := reflect.ValueOf(v)
    if rv.Kind() != reflect.Ptr || rv.IsNil() {
        return errors.New("must pass a pointer")
    }

    return c.unmarshalStruct(rv.Elem(), "")
}

func (c *configLoader) unmarshalStruct(rv reflect.Value, prefix string) error {
    rt := rv.Type()

    for i := 0; i < rt.NumField(); i++ {
        field := rt.Field(i)
        fv := rv.Field(i)

        // Get config key from tag
        tag := field.Tag.Get("config")
        if tag == "" {
            tag = strings.ToLower(field.Name)
        }

        key := tag
        if prefix != "" {
            key = prefix + "." + tag
        }

        // Check environment variable override
        if envKey := field.Tag.Get("env"); envKey != "" {
            if envVal := os.Getenv(envKey); envVal != "" {
                c.values[key] = envVal
            }
        }

        // Get value or default
        val, ok := c.values[key]
        if !ok {
            if def := field.Tag.Get("default"); def != "" {
                val = def
            }
        }

        // Set field value
        if val != nil {
            if err := c.setField(fv, val); err != nil {
                return fmt.Errorf("field %s: %w", key, err)
            }
        }

        // Validate
        if validateTag := field.Tag.Get("validate"); validateTag != "" {
            if err := c.validate(fv, validateTag); err != nil {
                return fmt.Errorf("validation failed for %s: %w", key, err)
            }
        }

        // Recurse into nested structs
        if fv.Kind() == reflect.Struct {
            if err := c.unmarshalStruct(fv, key); err != nil {
                return err
            }
        }
    }

    return nil
}
```

### File Watcher for Hot Reload
```go
type fileSource struct {
    path     string
    format   string
    watcher  *fsnotify.Watcher
    callback func()
}

func (f *fileSource) Watch(callback func()) error {
    watcher, err := fsnotify.NewWatcher()
    if err != nil {
        return err
    }

    if err := watcher.Add(f.path); err != nil {
        return err
    }

    f.watcher = watcher
    f.callback = callback

    go func() {
        for {
            select {
            case event := <-watcher.Events:
                if event.Op&fsnotify.Write == fsnotify.Write {
                    // Debounce rapid writes
                    time.Sleep(100 * time.Millisecond)
                    callback()
                }
            case err := <-watcher.Errors:
                log.Printf("watcher error: %v", err)
            }
        }
    }()

    return nil
}
```

### Usage Example
```go
func main() {
    cfg := config.New(
        config.WithFile("config.yaml"),
        config.WithEnvPrefix("APP"),
        config.WithDefaults(map[string]interface{}{
            "server.host": "localhost",
        }),
    )

    if err := cfg.Load(); err != nil {
        log.Fatal(err)
    }

    var serverCfg ServerConfig
    if err := cfg.Unmarshal(&serverCfg); err != nil {
        log.Fatal(err)
    }

    cfg.OnChange(func() {
        log.Println("Config changed, reloading...")
        cfg.Load()
        cfg.Unmarshal(&serverCfg)
    })
}
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Multi-Source Loading

**Summary:** Load configuration from files, env vars, and flags.

**Definition of Done:** All sources are loaded and merged correctly.

### Ticket 2: Validation

**Summary:** Validate configuration against schema.

**Definition of Done:** Invalid configs are rejected with clear errors.

### Ticket 3: Hot Reload

**Summary:** Watch config files and reload on change.

**Definition of Done:** Config changes are detected and applied.

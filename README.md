# 🇮🇳 Pizza Analysis — Telugu

Telugu text analysis plugin for [INFINI Pizza](https://github.com/pizza-rs/pizza).

## Pipeline

```
StandardTokenizer → IndicNormalization → TeluguNormalization → Lowercase
    → DecimalDigit → Stop → TeluguStem
```

## Components

| Component | Name | Description |
|-----------|------|-------------|
| Analyzer | `telugu` | Full Telugu analysis pipeline (NEW — core had no Telugu analyzer) |
| Filter | `telugu_normalization` | Telugu digit → ASCII, zero-width removal |
| Filter | `telugu_stop` | 90+ Telugu stop words |

## License

MIT

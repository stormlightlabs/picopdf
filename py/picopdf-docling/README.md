# picopdf-docling

`picopdf-docling` is the internal Python sidecar used by picopdf for Docling-based PDF reading. It is not a second user-facing picopdf CLI.

The conversion protocol will be implemented with structured PDF reading. The package currently exposes the version probe used to verify packaging and protocol compatibility:

```console
$ picopdf-docling --protocol-version
1
```

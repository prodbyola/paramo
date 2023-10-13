# Paramo
Paramo provides a simple and efficient way to compress and decompress files using the Huffman coding algorithm. Paramo aims to reduce the size of files while preserving data integrity.

## Features
- Huffman encoding for file compression.
- Huffman decoding for decompression.
- Command-line interface for easy compression and decompression.

## Usage
To use paramo, you can supply the following arguments.
- **--input | -i:** (required) The input file you wish to compress/decompress. If you're decompressing, this must be a file you compressed using Paramo.

- **--decode | -d:** (optional) Specifies whether paramo should operate in decoding mode. This is a boolean value with `false` as default.

- **--output | -o:** (optional) This is an optional *output folder* where you want Paramo to store your compressed/decompressed file. If not supplied, Paramo will use the same folder as the input.

- **--filename | -f:** (optional) An optional output filename to be used for the compressed/decompressed filename. If not supplied, filename will be the same as input. 

### Example
```bash
paramo -i sample.txt
```

The command above will produce a `sample.paramo` encoded file. To decode the encoded file, run Paramo in decoding mode and set the encoded file as input:
```bash
paramo -i sample.paramo -d
```

### Note on output extension
When compressing, output file extension will be `.paramo`. When decompressing, output file extension will same as original file.

## Roadmap
- Add more tests
- Add Process Monitor
- Fix padding issues
- Add LZMA2 implementation
- Folder compression
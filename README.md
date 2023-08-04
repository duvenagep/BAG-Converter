# NLX - Kadaster Bag Extract XML Parser 
![RUST](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)![polars](https://camo.githubusercontent.com/1fe374024cb4399bc5602bee62207cebbf17f7e6217f96668d2744f86e328667/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f7374796c653d666f722d7468652d6261646765266d6573736167653d506f6c61727326636f6c6f723d434437393243266c6f676f3d506f6c617273266c6f676f436f6c6f723d464646464646266c6162656c3d)


NLX is a open source tool to convert the Dutch LVBAG Landelijke Voorziening Basisregistratie Adressen en Gebouwen [lvbag-extract-nl](https://www.kadaster.nl/zakelijk/producten/adressen-en-gebouwen/bag-2.0-extract) dataset from XML to any desired spatial or text format. Many tools exist like GDAL `ogr2ogr`, however these tools are big dependancies and are sometimes not desired. 

NLX is a modern CLI completely written in rust with zero-copy deserialisation for blazingly fast performance and low memory overhead!

## Features

- Parses Kadaster Bag Extract XML files with high performance and accuracy.
- Converts XML data to user-defined output formats, such as JSON, CSV, or other structured data formats.
- Provides a user-friendly and intuitive command-line interface for easy interaction.
- Handles large XML files efficiently, making it suitable for processing extensive datasets.
- Can be easily integrated into existing data processing pipelines or scripts.

## Motivation

Kadaster's Bag Extract XML files contain essential geospatial data, but handling them efficiently can be challenging due to their size and complexity. NLX was developed to address this problem and empower users to extract specific information from these files easily. By using NLX, users can save time and effort while working with geospatial data provided by Kadaster.

The `lvbag-extract-nl` consists of a nested zip folder/file structure as seen below. The current total file size is `3,18 GB` when fully compressed and `99,97 GB` uncompressed. NLX is able to parse the entire extract in 5min with a max memory footprint of `3,24 GB`, most of which is simply loaded the zip into memory. 

```bash
lvbag-extract-nl.zip/
	├── 9999InOnderzoek08072023.zip
	├── 9999Inactief08072023.zip
	├── 9999LIG08072023.zip
	├── 9999NUM08072023.zip
	├── 9999NietBag08072023.zip
	├── 9999OPR08072023.zip
	├── 9999PND08072023.zip
	├── 9999STA08072023.zip
	├── 9999VBO08072023.zip
	├── 9999WPL08072023.zip
	├── GEM-WPL-RELATIE-08072023.zip
	└── Leveringsdocument-BAG-Extract.xml
```

## Usage

### Installation
#### Mac OS

To install NLX, the easiest way to install is using brew:

```
brew install nlx
```

### Getting Started

The CLI structure is as follows:
```
A general purpose open source CLI to convert the Dutch LVBAG data

Usage: nlx <COMMAND>

Commands:
  lvbag  Specify LVBAG (Basisregistratie Adressen en Gebouwen) BAG 2.0 Extract download URL and destination folder
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
The `parse` command has the following optional input variables:
```
Parse BAG 2.0 Extract

Usage: nlx lvbag parse [OPTIONS] -l <FILE>

Options:
  -i, --info                Info about files in the BAG 2.0 Extract to Parse
  -b [<BAG_OBJECT>...]      List of BAG objects that are Parsable,num_args = 0.., value_delimiter = ',', use_value_delimiter=true [possible values: vbo, opr, wpl, lig, pnd, num, sta]
  -l <FILE>                 Bag file to be parsed (eq. lvbag-extract-nl.zip)
  -f <FORMAT>               Supported Parse formats for LVBAG XML (POSTGIS, DSV, GEOJSON) [default: CSV]
  -p <PROJECTION>           Any supported projection (EPSG:4632, EPSG:28892) [default: EPSG:28892]
  -k <KEEP_COLUMNS>         List of columns to keep during parsing
  -e <EXCLUDE_COLUMNS>      List of columns to exclude during parsing
  -h, --help                Print help
```


Once NLX is installed, you can use it to parse Kadaster Bag Extract XML files and convert them to different formats. Here are some basic example of how to use NLX:

```
nlx lvbag parse -f CSV -p EPSG::4632 -l lvbag-extract-nl.zip
```
The above will parse the entire zip to CSV while converting the geometries from the default `EPSG::28892` to `EPSG::4632` 

```
nlx lvbag parse -b num sta lig -f CSV -p EPSG::4632 -l lvbag-extract-nl.zip
```


In this example, `input.xml` is the Kadaster Bag Extract XML file to parse, `-f json` specifies that the output format should be JSON, and `-o output.json` indicates the name of the output file.

### Supported Formats

NLX supports various output formats for conversion, such as:

- JSON/GeoJson
- CSV
- Shapefile
- GeoDatabase
- KML
- GeoJSON
- and more.

To see the full list of supported formats and how to use them, refer to the [documentation](https://github.com/duvenagep/BAG-Converter).

## Contributing

We welcome contributions to NLX! If you find a bug, have a feature request, or want to add enhancements, feel free to open an issue or submit a pull request on our [GitHub repository](https://github.com/duvenagep/BAG-Converter).

Please ensure that you follow the [code of conduct](https://example.com/nlx/docs/contributing) when contributing to this project.

## License

NLX is open-source software licensed under the [MIT License](https://example.com/nlx/LICENSE).

## Contact

For any questions, suggestions, or inquiries, you can reach the NLX team at duvenagep@gmail.com or join our community on [email](duvenagep@gmail.com).

---

We hope NLX proves to be a valuable tool in your geospatial data processing workflow. Happy parsing!

Best regards,

The NLX Team

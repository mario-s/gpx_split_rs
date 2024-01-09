# GPX file splitter

This tool can split a route or track in a GPX file into smaller chunks. It will create new files, where each one has a suffix to the filename like so _FILENAME_1.gpx_.

Inspired by my [earlier project](https://github.com/mario-s/gpx_split_py) which uses Python, but this time in Rust.

## Usage

The type of splitting is controlled with command line arguments. Here are some samples to cut a track or route.

- split a track after 500 points (default)<br/>
`gpx_split My_Track.gpx`
- split a route after 600 points<br/>
`gpx_split -t route My_Route.gpx -m 600`
- split a track after 50 km (unit is meter)<br/>
`gpx_split My_Track.gpx -b len -m 50000`
- split a route after 50 km<br/>
`gpx_split -t route My_Route.gpx -b len -m 50000`
- split a track at some locations with a maximum distance to a POI of 20 m (if argument n is not provided, the program will look for splitting points in the source file)<br/>
`gpx_split My_Track.gpx -b loc -m 20 -n POI.gpx`
- split a route at some locations with a maximum distance to a POI of 10 m<br/>
`gpx_split -t route My_Route.gpx -b loc -m 10 -n POI.gpx`

The points at a location for splitting must not be embedded in a track or route. For instance the content of POI.gpx would look like this:

```xml
<?xml version="1.0"?>
<gpx xmlns="http://www.topografix.com/GPX/1/1" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
creator="Desktop App" version="1.1"
xsi:schemaLocation="http://www.topografix.com/GPX/1/1 http://www.topografix.com/GPX/1/1/gpx.xsd">
<metadata>
    <link href="http://www.garmin.com">
      <text>Garmin International</text>
    </link>
    <time>2024-01-06T16:25:43Z</time>
    <bounds maxlat="44.025105033069849" maxlon="12.406672211363912" minlat="41.984547637403011" minlon="10.077079888433218"/>
  </metadata>
  <!--this is a splitting point for a track/route -->
  <wpt lat="42.178661786019802" lon="12.14837808161974">
    <ele>0</ele>
    <time>2021-05-11T10:38:41Z</time>
    <name>Agriturismo "Il marchese del grillo"</name>
    <sym>Restaurant</sym>
  </wpt>
</gpx>
```

### Help
`gpx_split --help`

### Verbose
For more information during the process, run the application with the environment variable `RUST_LOG`

`RUST_LOG=debug gpx_split <ARGS>`
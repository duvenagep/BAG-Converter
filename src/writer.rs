trait OutputWriter {
    fn write(&self, input: &str) -> Result<(), Box<dyn std::error::Error>>;
}

struct CSVWriter {}

impl OutputWriter for CSVWriter {
    fn write(&self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

struct GEOJSONWriter {
    // any specific config or dependencies for GEOJSON writing
}

impl OutputWriter for GEOJSONWriter {
    fn write(&self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

struct SHAPEFILEWriter {}

impl OutputWriter for SHAPEFILEWriter {
    fn write(&self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

struct KMLWriter {}

impl OutputWriter for KMLWriter {
    fn write(&self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

struct Writer<T: OutputWriter> {
    strategy: T,
}

impl<T: OutputWriter> Writer<T> {
    pub fn new(strategy: T) -> Self {
        Self { strategy }
    }

    pub fn write(&self, input: &str) {
        self.strategy.write(input);
    }
}

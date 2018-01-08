### MimeDetective

The `MimeDetective` spies for the magic number of a file or buffer
and spits out strongly typed Mimes.

#### Example
```
extern crate mime_detective;
use mime_detective::MimeDetective;

let detective = MimeDetective::new().unwrap();
let mime = detective.detect_filepath("Cargo.toml").unwrap();
```
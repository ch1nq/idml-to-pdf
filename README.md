# IDML parser
An IDML parser built using Rust, that will eventually be able to produce PDFs similar to ones exported directly from InDesign. 

Currenltly the parsing of IDML into Rust structures is done using [quick-xml](https://docs.rs/quick-xml/0.20.0/quick_xml/) and serde, while the generating of PDF's is done using [printpdf](https://github.com/fschutt/printpdf).

## Examples
No real examples at the moment 😢 but they will be added continuously, as more features are supported. However, take a look in the [test folder](./test), where you will find tests of generated PDFs using incomplete features. 

## Planned features
It should ideally support the full IDML Specification, but for now I am starting simple with getting basic examples up and running. 
Here are roughly what I will start by implementing:

- Unzip IDML file into a directory
- Parse IDML files into rust data structures
  - Spreads / MasterSpreads
  - Stories
  - Resources
- Render to a PDF
  - Blank spreads/pages
  - Simple geometry (Rectangles/Circles/Polygons)
  - TextFrames 
  - Integration of MasterSpreads
  - Images
  - Media objects (PDF/EPS/SVG)
  - Character/Paragraph styles

...and lots more. The full specification can be found [here](https://wwwimages.adobe.com/content/dam/acom/en/devnet/indesign/sdk/cs6/idml/idml-specification.pdf) 📖. 

## Interested?
If the project interests you or you want to contribute you can always make a pull request or contact me directly.

http://daske.dk
  

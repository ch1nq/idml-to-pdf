# idml-to-pdf
Building a IDML parser using Rust, that will eventually be able to produce PDFs similar ones exported directly from InDesign.

## Examples
No examples at the moment 😢 but they will be added continuously, as more features are supported. 

## Roadmap
It should ideally support the full IDML Specification, but for now I am starting simple with getting basic examples up and running. 
Here is a rough roadmap that I will try to follow:

- [x] Unzip IDML file into a directory
- [ ] Parse IDML files into rust data structures
  - [ ] Spreads / MasterSpreads
  - [ ] Stories
  - [ ] Resources
  - [ ]
  - [ ]
  
- [ ] Render to a PDF
  - [x] Blank spreads/pages
  - [x] Simple geometry (Rectangles/Circles/Polygons)
  - [ ] TextFrames 
  - [ ] Integration of MasterSpreads
  - [ ] Images
  - [ ] Media objects (PDF/EPS/SVG)
  - [ ] Character/Paragraph styles

...and lots more. The full specification can be found [here](https://wwwimages.adobe.com/content/dam/acom/en/devnet/indesign/sdk/cs6/idml/idml-specification.pdf) 📖. 

## Interested?
If the project interests you or you want to contribute you can always make a pull request or contact me directly.

http://daske.dk
  

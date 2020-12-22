use printpdf::{PdfDocumentReference, PdfPageReference, PdfLayerReference};
use printpdf::indices::{PdfLayerIndex, PdfPageIndex};

pub fn layer_from_index(
    pdf_doc: &PdfDocumentReference,
    page_index: &Option<PdfPageIndex>,
    layer_index: &Option<PdfLayerIndex>
) -> Result<PdfLayerReference, String> {
    // Get the current layer of the PDF we are working on
    match (page_index, layer_index) {
        (&Some(page_id), &Some(layer_id)) => {
            Ok(pdf_doc.get_page(page_id).get_layer(layer_id))
        },
        (&Some(_), &None) => Err("No layer index provided".to_string()),
        (&None, &Some(_)) => Err("No page index provided".to_string()),
        (&None, &None) => Err("No page and layer index provided".to_string()),
    }
}

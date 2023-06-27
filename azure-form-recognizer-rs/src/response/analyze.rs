#![allow(unused)]
use serde::Deserialize;

// AnalyzeDocumentResponse has no body.

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GetAnalyzeResultResponse {
    status: Option<String>,
    created_date_time: Option<String>,
    last_updated_date_time: Option<String>,
    analyze_result: Option<AnalyzeResult>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AnalyzeResult {
    api_version: Option<String>,
    model_id: Option<String>,
    string_index_type: Option<StringIndexType>,
    content: Option<String>,
    pages: Option<Vec<Page>>,
    tables: Option<Vec<Table>>,
    key_value_pairs: Option<Vec<KVPair>>,
    entities: Option<Vec<Entity>>,
    paragraphs: Option<Vec<Paragraph>>,
    styles: Option<Vec<Style>>,
    documents: Option<Vec<Document>>,
    languages: Option<Vec<Language>>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Page {
    page_number: Option<u32>,
    angle: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    unit: Option<String>,
    kind: Option<PageKind>,
    spans: Option<Vec<Span>>,
    words: Option<Vec<Word>>,
    selection_marks: Option<Vec<SelectionMark>>,
    lines: Option<Vec<Line>>,
    annotations: Option<Vec<Annotation>>,
    barcodes: Option<Vec<Barcode>>,
    formulas: Option<Vec<Formula>>,
    images: Option<Vec<Image>>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Table {
    row_count: Option<u32>,
    column_count: Option<u32>,
    bounding_regions: Option<Vec<BoundingRegion>>,
    spans: Option<Vec<Span>>,
    cells: Option<Vec<Cell>>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct KVPair {
    key: Option<Key>,
    value: Option<Value>,
    common_name: Option<String>,
    confidence: Option<f32>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Entity {
    category: Option<String>,

    #[serde(rename(deserialize = "subCategory"))]
    subcategory: Option<String>,
    content: Option<String>,
    bounding_regions: Option<Vec<BoundingRegion>>,
    spans: Option<Vec<Span>>,
    confidence: Option<f32>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Document {
    //todo
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SelectionMark {
    //todo
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Annotation {
    //todo
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Barcode {
    //todo
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Formula {
    //todo
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Image {
    //todo
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Span {
    offset: Option<u32>,
    length: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Word {
    content: Option<String>,
    polygon: Option<Vec<f32>>,
    confidence: Option<f32>,
    span: Option<Span>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Line {
    content: Option<String>,
    polygon: Option<Vec<f32>>,
    spans: Option<Vec<Span>>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Paragraph {
    spans: Option<Vec<Span>>,
    bounding_regions: Option<Vec<BoundingRegion>>,
    content: Option<String>
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BoundingRegion {
    page_number: Option<u32>,
    polygon: Vec<f32>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Style {
    is_handwritten: Option<bool>,
    spans: Option<Vec<Span>>,
    confidence: Option<f32>,
    similar_font_family: Option<String>,
    font_style: Option<String>,
    font_weight: Option<String>,
    color: Option<String>,
    background_color: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Language {
    spans: Option<Vec<Span>>,
    local: Option<String>,
    confidence: Option<f32>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Cell {
    //todo
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Key {
    content: Option<String>,
    bounding_regions: Option<Vec<BoundingRegion>>,
    spans: Option<Vec<Span>>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Value {
    content: Option<String>,
    bounding_regions: Option<Vec<BoundingRegion>>,
    spans: Option<Vec<Span>>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum StringIndexType {
    TextElements,
    UnicodeCodePoint,
    Utf16CodeUnit,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum PageKind {
    Document,
    Sheet,
    Slide,
    Image,
}
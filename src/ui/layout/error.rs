use taffy::TaffyError;
#[derive(Debug)]
pub enum LayoutError {
    Taffy(TaffyError),
    InvalidStyleName(String),
}

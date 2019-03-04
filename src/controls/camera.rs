use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum AxisControls {
    RotateHoriz,
    RotateVert,
    TranslateX,
    TranslateZ,
    Zoom,
}

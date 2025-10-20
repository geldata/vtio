//! Mouse mode control commands.

use vtenc::{ConstEncode, Encode, EncodeError, csi, write_csi};

use crate::terminal_mode;

terminal_mode!(
    #[doc = "Normal mouse tracking (send X & Y on button press/release)."]
    MouseTracking,
    1000
);

terminal_mode!(
    #[doc = "Button-event tracking (report button motion/dragging)."]
    MouseButtonEventTracking,
    1002
);

terminal_mode!(
    #[doc = "Any-event tracking (report all motion events)."]
    MouseAnyEventTracking,
    1003
);

terminal_mode!(
    #[doc = "RXVT mouse mode (extended coordinates >223, less preferred)."]
    MouseRxvtMode,
    1015
);

terminal_mode!(
    #[doc = "SGR mouse mode (extended coordinates >223, preferred)."]
    MouseSgrMode,
    1006
);

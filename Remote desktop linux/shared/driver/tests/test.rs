#[cfg(test)]
mod tests {
    use std::{io, thread::sleep, time::Duration};

    use evdev::{uinput::VirtualDevice, Device, EventType, InputEvent, KeyCode, RelativeAxisCode};
    use kmf_driver::{
        event::{DriverEvent, MouseButton, MouseMove},
        reader::DriverReader,
    };

    //use super::*;

    #[macro_export]
    macro_rules! attribset {
    ( $( $key:expr ),* $(,)? ) => {{
        let arr = [ $( $key ),* ];
        arr.iter().collect::<evdev::AttributeSet<_>>()
    }};
    }

    fn setup() -> io::Result<(VirtualDevice, VirtualDevice, Device, Device)> {
        use KeyCode as KC;
        use RelativeAxisCode as RAC;

        let keys = attribset![KC::KEY_A, KC::KEY_B];

        let mut keyboard = VirtualDevice::builder()?
            .name("reader-test-keyboard")
            .with_keys(&keys)?
            .build()?;

        let buttons = attribset![KC::BTN_LEFT, KC::BTN_RIGHT];
        let rel_axes = attribset![RAC::REL_X, RAC::REL_Y];

        let mut mouse = VirtualDevice::builder()?
            .name("reader-test-mouse")
            .with_keys(&buttons)?
            .with_relative_axes(&rel_axes)?
            .build()?;

        let path = keyboard.enumerate_dev_nodes_blocking()?.next().unwrap()?;
        let key_copy = match Device::open(path) {
            Ok(d) => d,
            Err(e) => {
                if e.kind() == io::ErrorKind::PermissionDenied {
                    return Err(e);
                }
                panic!("{e}");
            }
        };

        key_copy.set_nonblocking(true)?;

        let path = mouse.enumerate_dev_nodes_blocking()?.next().unwrap()?;

        let mouse_copy = match Device::open(path) {
            Ok(d) => d,
            Err(e) => {
                if e.kind() == io::ErrorKind::PermissionDenied {
                    return Err(e);
                }
                panic!("{e}");
            }
        };

        mouse_copy.set_nonblocking(true)?;

        Ok((keyboard, mouse, key_copy, mouse_copy))
    }

    #[test]
    fn reader() {
        use EventType as ET;
        use KeyCode as KC;
        use RelativeAxisCode as RAC;

        let Ok((mut keyboard, mut mouse, key_copy, mouse_copy)) = setup() else {
            return;
        };

        let mut reader = DriverReader::new(vec![mouse_copy, key_copy]).unwrap();

        let tk = ET::KEY.0;

        let a = KC::KEY_A.code();
        let b = KC::KEY_B.code();

        let down = 1;
        let up = 0;

        let d = true;
        let u = false;

        let ad = InputEvent::new(tk, a, down);
        let au = InputEvent::new(tk, a, up);

        let bd = InputEvent::new(tk, b, down);
        let bu = InputEvent::new(tk, b, up);

        let _kps = [
            DriverEvent::keyboard_press(a, d),
            DriverEvent::keyboard_press(b, d),
            DriverEvent::keyboard_press(a, u),
            DriverEvent::keyboard_press(b, u),
        ];

        keyboard.emit(&[ad, bd, au, bu]).unwrap();

        sleep(Duration::from_millis(10));

        let bl = KC::BTN_LEFT.code();
        let br = KC::BTN_RIGHT.code();
        let lb = MouseButton::Left;
        let rb = MouseButton::Right;

        let ld = InputEvent::new(tk, bl, down);
        let lu = InputEvent::new(tk, bl, up);

        let rd = InputEvent::new(tk, br, down);
        let ru = InputEvent::new(tk, br, up);

        mouse.emit(&[ld, rd, lu, ru]).unwrap();

        let _mbs = [
            DriverEvent::mouse_click(lb, d),
            DriverEvent::mouse_click(rb, d),
            DriverEvent::mouse_click(lb, u),
            DriverEvent::mouse_click(rb, u),
        ];

        let tr = ET::RELATIVE.0;

        let rx = RAC::REL_X.0;
        let ry = RAC::REL_Y.0;

        let xs = [5, 12, -7];
        let ys = [1, 8, 0];

        let xe = xs.map(|x| InputEvent::new(tr, rx, x));
        let ye = ys.map(|y| InputEvent::new(tr, ry, y));

        mouse.emit(&xe).unwrap();
        mouse.emit(&ye).unwrap();

        let mmove: MouseMove = xs
            .iter()
            .zip(ys.iter())
            .map(|(x, y)| MouseMove {
                x: *x,
                y: *y,
                wheel: 0,
            })
            .sum();

        println!("test 3");

        let es = reader.read_events().unwrap();

        assert!(!es.is_empty());

        let mut res = MouseMove::default();
        for e in es {
            let DriverEvent::MouseMove(mm) = e else {
                panic!("should've returned a MouseMove, got {e:?}");
            };
            res += mm;
        }

        assert_eq!(res, mmove);
    }
}

// async fn printcap(mut receiver: broadcast::Receiver<nwws_oi::Message>) {
//     while let Ok(msg) = receiver.recv().await {
//         // println!("ttaaii: {};", msg.ttaaii);
//         if &msg.ttaaii[..1] == "X" {
//             let x = extractxml(&msg.message);
//             if let Ok(alert) = oasiscap::Alert::from_str(x) {
//                 termlog::printcap(alert);
//             } else {
//                 println!("Failed to parse: {}", x);
//             }
//         }
//     }
// }

use std::str::FromStr;

fn parsecap(xml: &str) -> Option<oasiscap::v1dot2::Alert> {
    if let Ok(alert) = oasiscap::Alert::from_str(xml) {
        Some(alert.into_latest())
    } else {
        None
    }
}

pub fn extractxml(message: &str) -> &str {
    let startpos = message.find('<').unwrap_or(0);
    let endpos = message.rfind('>').unwrap_or(0);
    &message[startpos..=endpos]
}

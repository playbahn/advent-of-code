fn main() {
    let ip_list: Vec<String> = std::fs::read_to_string("input/day-07.txt")
        .unwrap()
        .lines()
        .map(|ip| ip.to_owned())
        .collect();

    let mut tls: u16 = 0;
    let mut ssl: u16 = 0;
    
    '_ip: for ip in ip_list {
        let (mut has_hyp_abba, mut marked_tls, mut marked_ssl): (bool, bool, bool) =
            (false, false, false);
        let (mut abas, mut babs): (Vec<&[u8]>, Vec<&[u8]>) = (Vec::new(), Vec::new());

        // &str converted to &[u8] with as_bytes just so that windows can be used

        '_seq: for (offset, seq) in ip.split(['[', ']']).enumerate() {
            if offset % 2 == 0 {
                // supernet sequence

                // part 1
                // skip if a past hypernet seq had any ABBAs
                if !has_hyp_abba {
                    let mut sup_seq_windows = seq.as_bytes().windows(4);
                    '_marked_tls: while !marked_tls {
                        if let Some(window_tls) = sup_seq_windows.next() {
                            // check ABBA
                            if window_tls[0] == window_tls[3]
                                && window_tls[1] == window_tls[2]
                                && window_tls[0] != window_tls[1]
                            {
                                tls += 1;
                                // so that in next `while !marked_tls` iteration, we do not check for ABBA
                                marked_tls = true;
                            }
                        } else {
                            break;
                        }
                    }
                }

                // checking for ABA, part 2
                let mut sup_seq_windows = seq.as_bytes().windows(3);
                '_marked_ssl_aba: while !marked_ssl {
                    if let Some(window_ssl) = sup_seq_windows.next() {
                        if window_ssl[0] == window_ssl[2] && window_ssl[1] != window_ssl[0] {
                            // for future checks
                            abas.push(window_ssl);
                            // check for corresponding BAB for window
                            if babs.contains(&&[window_ssl[1], window_ssl[0], window_ssl[1]][..]) {
                                // window is infact ssl
                                ssl += 1;
                                marked_ssl = true;
                            }
                        }
                    } else {
                        break;
                    }
                }
            } else /* if offset % 2 != 0 */ {
                // hypernet sequence

                // part 1
                let mut hyp_seq_windows = seq.as_bytes().windows(4);
                // skip if a past or current hypernet seq had or should get any ABBAs,
                // we do not need to check again
                '_has_hyp_abba: while !has_hyp_abba {
                    if let Some(window_tls) = hyp_seq_windows.next() {
                        // check ABBA
                        if window_tls[0] == window_tls[3]
                            && window_tls[1] == window_tls[2]
                            && window_tls[0] != window_tls[1]
                        {
                            // no need to check for ABBA, no next `while !has_hyp_abba {}` iteration
                            has_hyp_abba = true;
                            if marked_tls {
                                marked_tls = false;
                                tls -= 1;
                            }
                        }
                    } else {
                        break;
                    }
                }

                // checking for BAB, part 2
                let mut hyp_seq_windows = seq.as_bytes().windows(3);
                '_marked_ssl_bab: while !marked_ssl {
                    if let Some(window_ssl) = hyp_seq_windows.next() {
                        if window_ssl[0] == window_ssl[2] && window_ssl[1] != window_ssl[0] {
                            // for future checks
                            babs.push(window_ssl);
                            // check for corresponding BAB for window
                            if abas.contains(&&[window_ssl[1], window_ssl[0], window_ssl[1]][..]) {
                                // window is infact ssl
                                ssl += 1;
                                marked_ssl = true;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("tls: {tls} ssl: {ssl}");
}

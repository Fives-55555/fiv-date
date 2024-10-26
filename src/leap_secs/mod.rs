fn leap_sec_add(dur: &mut Duration, b: bool) {
    *dur -= Duration::from_secs(if b {
        27
    } else {
        match dur.as_secs() {
            ..=78796799 => 27,  //1972.06.30-23::59::59
            ..=94694399 => 26,  //1972.12.31-23::59::59
            ..=126230399 => 25, //1973.12.31-23::59::59
            ..=157766399 => 24, //1974.12.31-23::59::59
            ..=189302399 => 23, //1975.12.31-23::59::59
            ..=220924799 => 22, //1976.12.31-23::59::59
            ..=252460799 => 21, //1977.12.31-23::59::59
            ..=283996799 => 20, //1978.12.31-23::59::59
            ..=315532799 => 19, //1979.12.31-23::59::59
            ..=362793599 => 18, //1981.06.3-23::59::59
            ..=394329599 => 17, //1982.06.3-23::59::59
            ..=425865599 => 16, //1983.06.3-23::59::59
            ..=489023999 => 15, //1985.06.3-23::59::59
            ..=567993599 => 14, //1987.12.31-23::59::59
            ..=631151999 => 13, //1989.12.31-23::59::59
            ..=662687999 => 12, //1990.12.31-23::59::59
            ..=709948799 => 11, //1992.06.3-23::59::59
            ..=741484799 => 10, //1993.06.3-23::59::59
            ..=773020799 => 9,  //1994.06.3-23::59::59
            ..=820454399 => 8,  //1995.12.31-23::59::59
            ..=867715199 => 7,  //1997.06.3-23::59::59
            ..=915148799 => 6,  //1998.12.31-23::59::59
            ..=1136073599 => 5, //2005.12.31-23::59::59
            ..=1230767999 => 4, //2008.12.31-23::59::59
            ..=1341100799 => 3, //2012.06.3-23::59::59
            ..=1435708799 => 2, //2015.06.3-23::59::59
            ..=1483228799 => 1, //2016.12.31-23::59::59
            _ => unreachable!("Hä"),
        }
    })
}
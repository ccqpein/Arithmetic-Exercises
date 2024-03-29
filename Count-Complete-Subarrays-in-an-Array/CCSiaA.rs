pub fn count_complete_subarrays_bkp(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    use std::collections::HashSet;
    let s = nums.iter().cloned().collect::<HashSet<i32>>();
    let mut result = 0;

    let mut cache: HashSet<(usize, usize)> = HashSet::new();
    let mut i = 0;
    'a: while i < nums.len() {
        //'a: for mut i in 0..nums.len() {
        for mut how_many in s.len()..=nums.len() - i {
            //dbg!(i);
            //dbg!(how_many);
            dbg!(&cache);
            if cache.contains(&(i, how_many)) {
                continue;
            }
            let mut smap = HashMap::new();
            // let ss = nums[i..i + how_many]
            //     .iter()
            //     .cloned()
            //     .collect::<HashSet<i32>>();

            nums[i..i + how_many]
                .iter()
                .for_each(|e| *smap.entry(e).or_insert(0) += 1);
            let ss = smap.keys().map(|e| **e).collect::<HashSet<_>>();

            if ss.symmetric_difference(&s).count() == 0 {
                dbg!(i);
                dbg!(how_many);
                //dbg!(nums.len() - i - how_many + 1);
                result += 1;
                cache.insert((i, how_many));

                let temp = nums.len() - i - how_many + 1;
                dbg!(temp);
                //result += temp;
                for x in 1..nums.len() - i - how_many + 1 {
                    result += 1;
                    cache.insert((i, how_many + x));
                }

                while i < nums.len() {
                    match smap.get_mut(&nums[i]) {
                        Some(a) => {
                            if *a > 1 {
                                *a -= 1;
                                i += 1;
                                for ll in how_many - 1..nums.len() {
                                    result += 1;
                                    cache.insert((i, ll));
                                }
                                how_many -= 1;
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                    //dbg!(i);
                }

                //i += 1;
                continue 'a;
            }
        }
        i += 1;
    }
    result as i32
}

pub fn count_complete_subarrays2(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    use std::collections::HashSet;
    let s = nums.iter().cloned().collect::<HashSet<i32>>();

    let mut cache: HashSet<(usize, usize)> = HashSet::new();
    let mut i = 0;
    'a: while i < nums.len() {
        //'a: for mut i in 0..nums.len() {
        for mut j in i + s.len()..=nums.len() {
            println!("{i}, {j}");
            //dbg!(&cache);
            if cache.contains(&(i, j)) {
                continue;
            }

            let mut smap = HashMap::new();
            nums[i..j]
                .iter()
                .for_each(|e| *smap.entry(e).or_insert(0) += 1);
            let ss = smap.keys().map(|e| **e).collect::<HashSet<_>>();

            if ss.symmetric_difference(&s).count() == 0 {
                for jj in j..=nums.len() {
                    cache.insert((i, jj));
                }

                for ii in i..j {
                    match smap.get_mut(&nums[ii]) {
                        Some(a) => {
                            if *a > 1 {
                                *a -= 1;
                                for jj in j..=nums.len() {
                                    cache.insert((ii + 1, jj));
                                }
                            } else {
                                i = ii + 1;
                                continue 'a;
                                //break;
                            }
                        }
                        None => {
                            continue 'a;
                        }
                    }
                }
            }
        }
        i += 1
    }

    cache.len() as i32
}

pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    use std::collections::HashSet;
    let s = nums.iter().cloned().collect::<HashSet<i32>>();
    let mut result = 0;
    let mut left = 0;
    let mut cache: HashMap<i32, usize> = HashMap::new();

    for r in 0..nums.len() {
        *cache.entry(nums[r]).or_insert(0) += 1;
        while cache.keys().len() == s.len() {
            result += nums.len() - r;
            match cache.get_mut(&nums[left]) {
                Some(a) => *a -= 1,
                None => unreachable!(),
            }
            if cache.get(&nums[left]).unwrap() == &0_usize {
                cache.remove(&nums[left]);
            }
            left += 1;
        }
    }

    result as i32
}

fn main() {
    assert_eq!(4, count_complete_subarrays(vec![1, 3, 1, 2, 2]));
    assert_eq!(10, count_complete_subarrays(vec![5, 5, 5, 5]));
    assert_eq!(
        1,
        count_complete_subarrays(vec![254, 1690, 1690, 1068, 1779])
    );

    assert_eq!(3, count_complete_subarrays(vec![1786, 1786, 1786, 114]));

    assert_eq!(
        6,
        count_complete_subarrays(vec![1641, 448, 1641, 1437, 448, 1406, 1437])
    );

    assert_eq!(
        4,
        count_complete_subarrays(vec![1917, 1917, 608, 608, 1313, 751, 558, 1561, 608])
    );

    assert_eq!(
        7,
        count_complete_subarrays(vec![
            1762, 1998, 1762, 1762, 1762, 1348, 807, 1248, 1762, 1697, 189, 1998, 597, 1561, 1715,
            966, 1057, 1348, 1762, 181, 805, 649, 651, 732, 354, 825, 1, 1470, 1766, 232, 643, 966,
            1395, 1762, 1, 1087, 1150, 1741, 1117, 643, 238, 765, 827, 996, 1354, 739, 1573, 1622,
            232, 1931, 1650, 1395, 34, 550, 1087, 1470, 1598, 1647, 1915, 189, 1026, 980, 831,
            1859, 1145, 1414, 821, 1207, 1499, 1672, 1218, 765, 60, 550, 362, 458, 1762, 845, 1260,
            1667, 1867, 1614, 1482, 913, 1583, 1762, 1018, 782, 1348, 1314, 338, 14, 641, 1270,
            1184, 181, 444, 1046, 1695, 643, 238, 1108, 1715, 1682, 752, 444, 47, 596, 1019, 126,
            541, 1417, 1334, 550, 1626, 467, 1667, 828, 815, 841, 1889, 358, 550, 572, 1474, 482,
            494, 597, 250, 1439, 973, 1646, 1762, 232, 1741, 966, 181, 1280, 157, 512, 831, 685,
            409, 1614, 1622, 320, 1240, 827, 980, 104, 1667, 1246, 956, 482, 1288, 1832, 238, 603,
            820, 1579, 846, 1661, 609, 980, 1727, 405, 461, 223, 767, 1741, 739, 1098, 668, 447,
            933, 1724, 320, 1186, 1880, 1211, 1695, 282, 1353, 796, 469, 967, 232, 444, 561, 977,
            1793, 1302, 294, 1816, 182, 1334, 1176, 1260, 980, 1482, 1438, 643, 1297, 73, 1042,
            1859, 643, 1225, 181, 1943, 927, 1482, 1658, 744, 1482, 1019, 377, 1487, 1859, 1482,
            45, 867, 732, 872, 309, 412, 1584, 1530, 1766, 728, 470, 1353, 179, 940, 1248, 171,
            1335, 408, 1495, 624, 1602, 1682, 1762, 1302, 366, 1057, 746, 1667, 564, 134, 1128,
            1425, 739, 261, 19, 214, 1511, 1573, 562, 1134, 1271, 651, 893, 1667, 1662, 941, 1482,
            1150, 1998, 1832, 238, 656, 1179, 914, 1867, 1955, 755, 1276, 324, 1637
        ])
    );

    assert_eq!(
        7,
        count_complete_subarrays(vec![
            601, 601, 601, 613, 834, 446, 1137, 1366, 188, 549, 174, 601, 651, 843, 834, 137, 174,
            1375, 350, 1634, 446, 613, 555, 454, 843, 613, 1372, 1158, 174, 1425, 1715, 1854, 1372,
            863, 1408, 441, 925, 1759, 1969, 226, 1629, 1935, 458, 1035, 1969, 1479, 613, 1372,
            596, 1211, 601, 1361, 1128, 445, 186, 613, 479, 893, 1997, 1851, 1634, 1171, 335, 1158,
            834, 1372, 1481, 215, 1171, 856, 44, 940, 1786, 767, 744, 1752, 788, 135, 1881, 1006,
            651, 1183, 555, 900, 802, 1138, 1236, 1109, 894, 863, 73, 1006, 299, 641, 1977, 1350,
            753, 346, 206, 1297, 1445, 1361, 15, 802, 1003, 959, 905, 555, 1841, 885, 1244, 1496,
            1689, 582, 1772, 1773, 177, 1479, 1307, 1199, 478, 651, 1716, 925, 321, 1457, 545,
            1801, 1850, 419, 119, 1752, 321, 1702, 1927, 1801, 900, 1634, 1880, 1843, 1618, 1242,
            1372, 863, 574, 1997, 214, 839, 843, 44, 1039, 952, 1773, 1530, 1425, 893, 1280, 1152,
            1382, 1282, 1752, 794, 125, 1432, 585, 760, 585, 955, 1669, 566, 858, 1890, 681, 1205,
            168, 1644, 1280, 574, 798, 613, 856, 1157, 1963, 1726, 1154, 545, 784, 505, 858, 186,
            1663, 1290, 96, 198, 13, 615, 363, 1186, 1715, 1724, 681, 1977, 1971, 1672, 1773, 985,
            419, 807, 15, 1218, 993, 1672, 1297, 1004, 1775, 1821, 847, 1115, 1069, 1634, 347,
            1541, 161, 1119, 706, 412, 478, 943, 656, 1773, 633, 576, 33, 856, 1634, 1085, 474,
            127, 1466, 1818, 1371, 1631, 1475, 585, 1656, 1046, 605, 1957, 353, 1401, 988, 428,
            1160, 1775, 385, 1048, 749, 1481, 1359, 1971, 1827, 944, 726, 1085, 125, 1394, 1298,
            1224, 816, 174, 1530, 827, 636, 1270, 1198, 390, 1774, 430, 681, 19, 969, 155, 1077,
            749, 744, 606, 802, 740, 1144, 155, 1825, 576, 834, 865, 1867, 578, 268, 972, 82, 1936,
            319, 1183, 1147, 125, 576, 125, 96, 1466, 1366, 870, 947
        ])
    );

    assert_eq!(
        9,
        count_complete_subarrays(vec![
            589, 589, 1278, 1156, 1156, 884, 1501, 1016, 974, 954, 399, 1051, 1398, 1535, 1525,
            589, 1535, 342, 1507, 868, 1016, 868, 1068, 1413, 794, 1874, 502, 148, 1845, 342, 974,
            342, 433, 342, 1007, 1811, 1156, 1535, 1395, 1619, 1051, 1918, 502, 1024, 1188, 995,
            1167, 399, 1920, 1005, 1556, 637, 1611, 448, 1644, 550, 425, 846, 1739, 1642, 329, 353,
            353, 421, 1105, 1806, 1942, 810, 891, 1921, 387, 884, 1043, 995, 846, 1739, 1336, 1419,
            503, 342, 1582, 553, 425, 1535, 1597, 1543, 289, 1902, 200, 1638, 1646, 475, 1020,
            1188, 1141, 433, 724, 472, 1942, 489, 253, 1570, 549, 1506, 443, 801, 883, 1051, 1166,
            502, 1465, 1638, 502, 425, 1874, 1546, 56, 1756, 185, 1413, 767, 645, 405, 533, 637,
            268, 24, 995, 846, 1682, 44, 546, 917, 1970, 43, 1413, 1969, 989, 1739, 501, 1594, 967,
            1634, 1005, 1072, 364, 1543, 202, 277, 1581, 1506, 269, 1739, 119, 1447, 1005, 868,
            1992, 1535, 1273, 479, 1902, 20, 268, 1845, 1528, 1858, 1212, 527, 637, 222, 1810,
            1925, 1996, 1819, 1501, 1240, 119, 1167, 1753, 603, 551, 130, 1348, 1405, 1767, 198,
            842, 1809, 142, 1020, 1137, 868, 603, 605, 1122, 1075, 1197, 223, 1080, 1883, 329,
            1037, 1691, 800, 313, 1405, 375, 396, 311, 251, 804, 1543, 917, 1968, 589, 1364, 1217,
            465, 1546, 282, 245, 1254, 1588, 1848, 1360, 19, 1788, 1293, 225, 405, 1212, 135, 409,
            945, 1962, 405, 1702, 1107, 1413, 564, 1383, 1190, 1019, 849, 529, 862, 425, 1915,
            1481, 1110, 387, 693, 1732, 425, 1284, 1313, 100, 443, 744, 259, 450, 1540, 1210, 245,
            489, 1835, 1594, 280, 623, 780, 1713, 1075, 1976, 710, 1549, 519, 545, 1176, 1632, 104,
            1173, 293, 119, 1456, 1710, 1398, 1794, 962, 412, 868, 1156, 1105, 1806, 1546, 1242,
            490, 604, 1929, 306, 929, 397, 362, 311, 654, 1319, 1459, 1344, 984, 974, 189, 240,
            995, 865, 1506, 453, 946, 540, 1393, 358, 423, 1267, 245, 405
        ])
    );
}

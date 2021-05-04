use std::collections::HashMap;

pub fn get_colors() -> HashMap<u8, [u8; 3]> {
    let mut colortab = HashMap::new();
    colortab.insert(4, [89, 125, 39]);
    colortab.insert(5, [109, 153, 48]);
    colortab.insert(6, [127, 178, 56]);
    colortab.insert(7, [67, 94, 29]);
    colortab.insert(8, [174, 164, 115]);
    colortab.insert(9, [213, 201, 140]);
    colortab.insert(10, [247, 233, 163]);
    colortab.insert(11, [130, 123, 86]);
    colortab.insert(12, [140, 140, 140]);
    colortab.insert(13, [171, 171, 171]);
    colortab.insert(14, [199, 199, 199]);
    colortab.insert(15, [105, 105, 105]);
    colortab.insert(16, [180, 0, 0]);
    colortab.insert(17, [220, 0, 0]);
    colortab.insert(18, [255, 0, 0]);
    colortab.insert(19, [135, 0, 0]);
    colortab.insert(20, [112, 112, 180]);
    colortab.insert(21, [138, 138, 220]);
    colortab.insert(22, [160, 160, 255]);
    colortab.insert(23, [84, 84, 135]);
    colortab.insert(24, [117, 117, 117]);
    colortab.insert(25, [144, 144, 144]);
    colortab.insert(26, [167, 167, 167]);
    colortab.insert(27, [88, 88, 88]);
    colortab.insert(28, [0, 87, 0]);
    colortab.insert(29, [0, 106, 0]);
    colortab.insert(30, [0, 124, 0]);
    colortab.insert(31, [0, 65, 0]);
    colortab.insert(32, [180, 180, 180]);
    colortab.insert(33, [220, 220, 220]);
    colortab.insert(34, [255, 255, 255]);
    colortab.insert(35, [135, 135, 135]);
    colortab.insert(36, [115, 118, 129]);
    colortab.insert(37, [141, 144, 158]);
    colortab.insert(38, [164, 168, 184]);
    colortab.insert(39, [86, 88, 97]);
    colortab.insert(40, [106, 76, 54]);
    colortab.insert(41, [130, 94, 66]);
    colortab.insert(42, [151, 109, 77]);
    colortab.insert(43, [79, 57, 40]);
    colortab.insert(44, [79, 79, 79]);
    colortab.insert(45, [96, 96, 96]);
    colortab.insert(46, [112, 112, 112]);
    colortab.insert(47, [59, 59, 59]);
    colortab.insert(48, [45, 45, 180]);
    colortab.insert(49, [55, 55, 220]);
    colortab.insert(50, [64, 64, 255]);
    colortab.insert(51, [33, 33, 135]);
    colortab.insert(52, [100, 84, 50]);
    colortab.insert(53, [123, 102, 62]);
    colortab.insert(54, [143, 119, 72]);
    colortab.insert(55, [75, 63, 38]);
    colortab.insert(56, [180, 177, 172]);
    colortab.insert(57, [220, 217, 211]);
    colortab.insert(58, [255, 252, 245]);
    colortab.insert(59, [135, 133, 129]);
    colortab.insert(60, [152, 89, 36]);
    colortab.insert(61, [186, 109, 44]);
    colortab.insert(62, [216, 127, 51]);
    colortab.insert(63, [114, 67, 27]);
    colortab.insert(64, [125, 53, 152]);
    colortab.insert(65, [153, 65, 186]);
    colortab.insert(66, [178, 76, 216]);
    colortab.insert(67, [94, 40, 114]);
    colortab.insert(68, [72, 108, 152]);
    colortab.insert(69, [88, 132, 186]);
    colortab.insert(70, [102, 153, 216]);
    colortab.insert(71, [54, 81, 114]);
    colortab.insert(72, [161, 161, 36]);
    colortab.insert(73, [197, 197, 44]);
    colortab.insert(74, [229, 229, 51]);
    colortab.insert(75, [121, 121, 27]);
    colortab.insert(76, [89, 144, 17]);
    colortab.insert(77, [109, 176, 21]);
    colortab.insert(78, [127, 204, 25]);
    colortab.insert(79, [67, 108, 13]);
    colortab.insert(80, [170, 89, 116]);
    colortab.insert(81, [208, 109, 142]);
    colortab.insert(82, [242, 127, 165]);
    colortab.insert(83, [128, 67, 87]);
    colortab.insert(84, [53, 53, 53]);
    colortab.insert(85, [65, 65, 65]);
    colortab.insert(86, [76, 76, 76]);
    colortab.insert(87, [40, 40, 40]);
    colortab.insert(88, [108, 108, 108]);
    colortab.insert(89, [132, 132, 132]);
    colortab.insert(90, [153, 153, 153]);
    colortab.insert(91, [81, 81, 81]);
    colortab.insert(92, [53, 89, 108]);
    colortab.insert(93, [65, 109, 132]);
    colortab.insert(94, [76, 127, 153]);
    colortab.insert(95, [40, 67, 81]);
    colortab.insert(96, [89, 44, 125]);
    colortab.insert(97, [109, 54, 153]);
    colortab.insert(98, [127, 63, 178]);
    colortab.insert(99, [67, 33, 94]);
    colortab.insert(100, [36, 53, 125]);
    colortab.insert(101, [44, 65, 153]);
    colortab.insert(102, [51, 76, 178]);
    colortab.insert(103, [27, 40, 94]);
    colortab.insert(104, [72, 53, 36]);
    colortab.insert(105, [88, 65, 44]);
    colortab.insert(106, [102, 76, 51]);
    colortab.insert(107, [54, 40, 27]);
    colortab.insert(108, [72, 89, 36]);
    colortab.insert(109, [88, 109, 44]);
    colortab.insert(110, [102, 127, 51]);
    colortab.insert(111, [54, 67, 27]);
    colortab.insert(112, [108, 36, 36]);
    colortab.insert(113, [132, 44, 44]);
    colortab.insert(114, [153, 51, 51]);
    colortab.insert(115, [81, 27, 27]);
    colortab.insert(116, [17, 17, 17]);
    colortab.insert(117, [21, 21, 21]);
    colortab.insert(118, [25, 25, 25]);
    colortab.insert(119, [13, 13, 13]);
    colortab.insert(120, [176, 168, 54]);
    colortab.insert(121, [215, 205, 66]);
    colortab.insert(122, [250, 238, 77]);
    colortab.insert(123, [132, 126, 40]);
    colortab.insert(124, [64, 154, 150]);
    colortab.insert(125, [79, 188, 183]);
    colortab.insert(126, [92, 219, 213]);
    colortab.insert(127, [48, 115, 112]);
    colortab.insert(128, [52, 90, 180]);
    colortab.insert(129, [63, 110, 220]);
    colortab.insert(130, [74, 128, 255]);
    colortab.insert(131, [39, 67, 135]);
    colortab.insert(132, [0, 153, 40]);
    colortab.insert(133, [0, 187, 50]);
    colortab.insert(134, [0, 217, 58]);
    colortab.insert(135, [0, 114, 30]);
    colortab.insert(136, [91, 60, 34]);
    colortab.insert(137, [111, 74, 42]);
    colortab.insert(138, [129, 86, 49]);
    colortab.insert(139, [68, 45, 25]);
    colortab.insert(140, [79, 1, 0]);
    colortab.insert(141, [96, 1, 0]);
    colortab.insert(142, [112, 2, 0]);
    colortab.insert(143, [59, 1, 0]);
    colortab.insert(144, [147, 124, 113]);
    colortab.insert(145, [180, 152, 138]);
    colortab.insert(146, [209, 177, 161]);
    colortab.insert(147, [110, 93, 85]);
    colortab.insert(148, [112, 57, 25]);
    colortab.insert(149, [137, 70, 31]);
    colortab.insert(150, [159, 82, 36]);
    colortab.insert(151, [84, 43, 19]);
    colortab.insert(152, [105, 61, 76]);
    colortab.insert(153, [128, 75, 93]);
    colortab.insert(154, [149, 87, 108]);
    colortab.insert(155, [78, 46, 57]);
    colortab.insert(156, [79, 76, 97]);
    colortab.insert(157, [96, 93, 119]);
    colortab.insert(158, [112, 108, 138]);
    colortab.insert(159, [59, 57, 73]);
    colortab.insert(160, [131, 93, 25]);
    colortab.insert(161, [160, 114, 31]);
    colortab.insert(162, [186, 133, 36]);
    colortab.insert(163, [98, 70, 19]);
    colortab.insert(164, [72, 82, 37]);
    colortab.insert(165, [88, 100, 45]);
    colortab.insert(166, [103, 117, 53]);
    colortab.insert(167, [54, 61, 28]);
    colortab.insert(168, [112, 54, 55]);
    colortab.insert(169, [138, 66, 67]);
    colortab.insert(170, [160, 77, 78]);
    colortab.insert(171, [84, 40, 41]);
    colortab.insert(172, [40, 28, 24]);
    colortab.insert(173, [49, 35, 30]);
    colortab.insert(174, [57, 41, 35]);
    colortab.insert(175, [30, 21, 18]);
    colortab.insert(176, [95, 75, 69]);
    colortab.insert(177, [116, 92, 84]);
    colortab.insert(178, [135, 107, 98]);
    colortab.insert(179, [71, 56, 51]);
    colortab.insert(180, [61, 64, 64]);
    colortab.insert(181, [75, 79, 79]);
    colortab.insert(182, [87, 92, 92]);
    colortab.insert(183, [46, 48, 48]);
    colortab.insert(184, [86, 51, 62]);
    colortab.insert(185, [105, 62, 75]);
    colortab.insert(186, [122, 73, 88]);
    colortab.insert(187, [64, 38, 46]);
    colortab.insert(188, [53, 43, 64]);
    colortab.insert(189, [65, 53, 79]);
    colortab.insert(190, [76, 62, 92]);
    colortab.insert(191, [40, 32, 48]);
    colortab.insert(192, [53, 35, 24]);
    colortab.insert(193, [65, 43, 30]);
    colortab.insert(194, [76, 50, 35]);
    colortab.insert(195, [40, 26, 18]);
    colortab.insert(196, [53, 57, 29]);
    colortab.insert(197, [65, 70, 36]);
    colortab.insert(198, [76, 82, 42]);
    colortab.insert(199, [40, 43, 22]);
    colortab.insert(200, [100, 42, 32]);
    colortab.insert(201, [122, 51, 39]);
    colortab.insert(202, [142, 60, 46]);
    colortab.insert(203, [75, 31, 24]);
    colortab.insert(204, [26, 15, 11]);
    colortab.insert(205, [31, 18, 13]);
    colortab.insert(206, [37, 22, 16]);
    colortab.insert(207, [19, 11, 8]);
    return colortab;
}
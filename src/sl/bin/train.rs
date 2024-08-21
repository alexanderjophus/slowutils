const LOCO_10: &str = "      ====        ________                ___________ ";
const LOCO_11: &str = "  _D _|  |_______/        \\__I_I_____===__|_________| ";
const LOCO_12: &str = "   |(_)---  |   H\\________/ |   |        =|___ ___|   ";
const LOCO_13: &str = "   /     |  |   H  |  |     |   |         ||_| |_||   ";
const LOCO_14: &str = "  |      |  |   H  |__--------------------| [___] |   ";
const LOCO_15: &str = "  | ________|___H__/__|_____/[][]~\\_______|       |   ";
const LOCO_16: &str = "  |/ |   |-----------I_____I [][] []  D   |=======|__ ";

pub const LOCO_COLLECTION: [&str; 7] = [
    LOCO_10, LOCO_11, LOCO_12, LOCO_13, LOCO_14, LOCO_15, LOCO_16,
];

const WHEEL_10: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const WHEEL_11: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const WHEEL_12: &str = "  \\_/      \\O=====O=====O=====O_/      \\_/            ";

const WHEEL_20: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const WHEEL_21: &str = " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        ";
const WHEEL_22: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const WHEEL_30: &str = "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__ ";
const WHEEL_31: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const WHEEL_32: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const WHEEL_40: &str = "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__ ";
const WHEEL_41: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const WHEEL_42: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const WHEEL_50: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const WHEEL_51: &str = " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        ";
const WHEEL_52: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const WHEEL_60: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const WHEEL_61: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const WHEEL_62: &str = "  \\_/      \\_O=====O=====O=====O/      \\_/            ";

const WHEELS_10: [&str; 6] = [WHEEL_10, WHEEL_20, WHEEL_30, WHEEL_40, WHEEL_50, WHEEL_60];
const WHEELS_11: [&str; 6] = [WHEEL_11, WHEEL_21, WHEEL_31, WHEEL_41, WHEEL_51, WHEEL_61];
const WHEELS_12: [&str; 6] = [WHEEL_12, WHEEL_22, WHEEL_32, WHEEL_42, WHEEL_52, WHEEL_62];

pub const WHEEL_COLLECTION: [[&str; 6]; 3] = [WHEELS_10, WHEELS_11, WHEELS_12];

const COAL10: &str = "                              ";
const COAL11: &str = "                              ";
const COAL12: &str = "    _________________         ";
const COAL13: &str = "   _|                \\_____A  ";
const COAL14: &str = " =|                        |  ";
const COAL15: &str = " -|                        |  ";
const COAL16: &str = "__|________________________|_ ";
const COAL17: &str = "|__________________________|_ ";
const COAL18: &str = "   |_D__D__D_|  |_D__D__D_|   ";
const COAL19: &str = "    \\_/   \\_/    \\_/   \\_/    ";

pub const COAL_COLLECTION: [&str; 10] = [
    COAL10, COAL11, COAL12, COAL13, COAL14, COAL15, COAL16, COAL17, COAL18, COAL19,
];

use serialport::TTYPort;
use std::{
    collections::HashMap, io::Write, sync::{Arc, LazyLock, RwLock}
};

static COM: LazyLock<Arc<RwLock<TTYPort>>> = LazyLock::new(|| {
    let serial_port_builder = serialport::new("/dev/ttyS0", 9600)
        .data_bits(serialport::DataBits::Eight)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .timeout(std::time::Duration::from_millis(1000));
    Arc::new(RwLock::new(serial_port_builder.open_native().unwrap()))
});

static KEYS: LazyLock<HashMap<String, Vec<String>>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD04]".to_string());
    t.push("[kU04]".to_string());
    map.insert("a".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD05]".to_string());
    t.push("[kU05]".to_string());
    map.insert("b".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD06]".to_string());
    t.push("[kU06]".to_string());
    map.insert("c".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD07]".to_string());
    t.push("[kU07]".to_string());
    map.insert("d".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD08]".to_string());
    t.push("[kU08]".to_string());
    map.insert("e".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD09]".to_string());
    t.push("[kU09]".to_string());
    map.insert("f".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD0A]".to_string());
    t.push("[kU0A]".to_string());
    map.insert("g".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD0B]".to_string());
    t.push("[kU0B]".to_string());
    map.insert("h".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD0C]".to_string());
    t.push("[kU0C]".to_string());
    map.insert("i".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD0D]".to_string());
    t.push("[kU0D]".to_string());
    map.insert("j".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD0E]".to_string());
    t.push("[kU0E]".to_string());
    map.insert("k".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD0F]".to_string());
    t.push("[kU0F]".to_string());
    map.insert("l".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD10]".to_string());
    t.push("[kU10]".to_string());
    map.insert("m".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD11]".to_string());
    t.push("[kU11]".to_string());
    map.insert("n".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD12]".to_string());
    t.push("[kU12]".to_string());
    map.insert("o".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD13]".to_string());
    t.push("[kU13]".to_string());
    map.insert("p".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD14]".to_string());
    t.push("[kU14]".to_string());
    map.insert("q".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD15]".to_string());
    t.push("[kU15]".to_string());
    map.insert("r".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD16]".to_string());
    t.push("[kU16]".to_string());
    map.insert("s".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD17]".to_string());
    t.push("[kU17]".to_string());
    map.insert("t".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD18]".to_string());
    t.push("[kU18]".to_string());
    map.insert("u".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD19]".to_string());
    t.push("[kU19]".to_string());
    map.insert("v".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD1A]".to_string());
    t.push("[kU1A]".to_string());
    map.insert("w".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD1B]".to_string());
    t.push("[kU1B]".to_string());
    map.insert("x".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD1C]".to_string());
    t.push("[kU1C]".to_string());
    map.insert("y".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD1D]".to_string());
    t.push("[kU1D]".to_string());
    map.insert("z".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD1E]".to_string());
    t.push("[kU1E]".to_string());
    map.insert("1".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD1F]".to_string());
    t.push("[kU1F]".to_string());
    map.insert("2".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD20]".to_string());
    t.push("[kU20]".to_string());
    map.insert("3".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD21]".to_string());
    t.push("[kU21]".to_string());
    map.insert("4".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD22]".to_string());
    t.push("[kU2]".to_string());
    map.insert("5".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD23]".to_string());
    t.push("[kU23]".to_string());
    map.insert("6".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD24]".to_string());
    t.push("[kU24]".to_string());
    map.insert("7".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD25]".to_string());
    t.push("[kU25]".to_string());
    map.insert("8".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD26]".to_string());
    t.push("[kU26]".to_string());
    map.insert("9".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD27]".to_string());
    t.push("[kU27]".to_string());
    map.insert("0".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD28]".to_string());
    t.push("[kU28]".to_string());
    map.insert("enter".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD2A]".to_string());
    t.push("[kU2A]".to_string());
    map.insert("backspace".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD2B]".to_string());
    t.push("[kU2B]".to_string());
    map.insert("tab".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD2C]".to_string());
    t.push("[kU2C]".to_string());
    map.insert("space".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD2D]".to_string());
    t.push("[kU2D]".to_string());
    map.insert("-".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD2D]".to_string());
    t.push("[kU2D]".to_string());
    t.push("[kUE5]".to_string());
    map.insert("_".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD2E]".to_string());
    t.push("[kU2E]".to_string());
    map.insert("=".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD2E]".to_string());
    t.push("[kU2E]".to_string());
    t.push("[kUE5]".to_string());
    map.insert("+".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD2F]".to_string());
    t.push("[kU2F]".to_string());
    map.insert("[".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD2F]".to_string());
    t.push("[kU2F]".to_string());
    t.push("[kUE5]".to_string());
    map.insert("{".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD30]".to_string());
    t.push("[kU30]".to_string());
    map.insert("]".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD30]".to_string());
    t.push("[kU30]".to_string());
    t.push("[kUE5]".to_string());
    map.insert("}".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD31]".to_string());
    t.push("[kU31]".to_string());
    map.insert("\\".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD31]".to_string());
    t.push("[kU31]".to_string());
    t.push("[kUE5]".to_string());
    map.insert("|".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD33]".to_string());
    t.push("[kU33]".to_string());
    map.insert(";".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD33]".to_string());
    t.push("[kU33]".to_string());
    t.push("[kUE5]".to_string());
    map.insert(":".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD34]".to_string());
    t.push("[kU34]".to_string());
    map.insert("'".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD34]".to_string());
    t.push("[kU34]".to_string());
    t.push("[kUE5]".to_string());
    map.insert("\"".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD36]".to_string());
    t.push("[kU36]".to_string());
    map.insert(",".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD36]".to_string());
    t.push("[kU36]".to_string());
    t.push("[kUE5]".to_string());
    map.insert("<".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD37]".to_string());
    t.push("[kU37]".to_string());
    map.insert(".".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD37]".to_string());
    t.push("[kU37]".to_string());
    t.push("[kUE5]".to_string());
    map.insert(">".to_string(), t);

    let mut t = Vec::<String>::with_capacity(2);
    t.push("[kD38]".to_string());
    t.push("[kU38]".to_string());
    map.insert("/".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD38]".to_string());
    t.push("[kU38]".to_string());
    t.push("[kUE5]".to_string());
    map.insert("?".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD25]".to_string());
    t.push("[kU25]".to_string());
    t.push("[kUE5]".to_string());
    map.insert("*".to_string(), t);

    let mut t = Vec::<String>::with_capacity(4);
    t.push("[kDE5]".to_string());
    t.push("[kD20]".to_string());
    t.push("[kU20]".to_string());
    t.push("[kUE5]".to_string());
    map.insert("#".to_string(), t);

    map
});
static MOUSE: LazyLock<String> = LazyLock::new(|| "Hello, world!".to_string());

fn main() {
    let context = r#"#pragma execution_character_set("utf-8")

#include "frmipaddress.h"
#include "ui_frmipaddress.h"
#include "qdebug.h"

frmIPAddress::frmIPAddress(QWidget *parent) : QWidget(parent), ui(new Ui::frmIPAddress)
{
    ui->setupUi(this);
    on_btnSetIP_clicked();
}

frmIPAddress::~frmIPAddress()
{
    delete ui;
}

void frmIPAddress::on_btnSetIP_clicked()
{
    ui->widgetIP->setIP("192.168.1.56");
}

void frmIPAddress::on_btnGetIP_clicked()
{
    qDebug() << ui->widgetIP->getIP();
}

void frmIPAddress::on_btnClear_clicked()
{
    ui->widgetIP->clear();
}
"#;

    let mut com = COM.write().unwrap();
    let context_row = context.split("\n").collect::<Vec<_>>();
    for row in context_row {
        for ele in row.chars() {
            let key = format!("{}", ele);
            if let Some(value) = KEYS.get(&key) {
                if value.len() == 4 {
                    let _ = com.write(value[0].as_bytes());
                    std::thread::sleep(std::time::Duration::from_millis(200));
                    let _ = com.write(value[1].as_bytes());
                    std::thread::sleep(std::time::Duration::from_millis(200));
                    let _ = com.write(value[2].as_bytes());
                    std::thread::sleep(std::time::Duration::from_millis(200));
                    let _ = com.write(value[3].as_bytes());
                } else {
                    let _ = com.write(value[0].as_bytes());
                    std::thread::sleep(std::time::Duration::from_millis(200));
                    let _ = com.write(value[1].as_bytes());
                }
            }
        }

        let value = KEYS.get("enter").unwrap();
        let _ = com.write(value[0].as_bytes());
        std::thread::sleep(std::time::Duration::from_millis(200));
        let _ = com.write(value[1].as_bytes());
    }

    // println!("{:?}", context_row[0]);
}

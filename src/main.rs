use serialport::COMPort;
use std::{
    collections::HashMap,
    io::Write,
    sync::{Arc, LazyLock, RwLock},
};

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
    let context = r#"#include "ttkgiflabelwidget.h"

#include <QTimer>
#include <QPainter>

#define GIF_BALLON_WHITE        35
#define GIF_CICLE_BLUE          58
#define GIF_RICE_FONT_WHITE     26
#define GIF_RICE_FONT_BLACK_BIG 42
#define GIF_RICE_FONT_BLACK     16
#define GIF_HOURGLASS_WHITE     38
#define GIF_RADIO_BLUE          14
#define GIF_CHECK_BLUE          93
#define GIF_RECORD_RED          30
#define GIF_CLOSE_WHITE_WIDTH   350
#define GIF_CLOSE_WHITE_HEIGHT  50

TTKGifLabelWidget::TTKGifLabelWidget(QWidget *parent)
    : QLabel(parent)
{
    m_index = -1;
    m_isRunning = false;
    m_infinited = true;

    setFixedSize(GIF_CICLE_BLUE, GIF_CICLE_BLUE);

    m_timer = new QTimer(this);
    m_timer->setInterval(100);
    connect(m_timer, SIGNAL(timeout()), SLOT(timeout()));
}

TTKGifLabelWidget::TTKGifLabelWidget(Type type, QWidget *parent)
    : TTKGifLabelWidget(parent)
{
    setType(type);
}

TTKGifLabelWidget::~TTKGifLabelWidget()
{
    delete m_timer;
}

void TTKGifLabelWidget::setType(Type type)
{
    switch(m_type = type)
    {
        case Gif_Ballon_White: setFixedSize(GIF_BALLON_WHITE, GIF_BALLON_WHITE); break;
        case Gif_Cicle_Blue: setFixedSize(GIF_CICLE_BLUE, GIF_CICLE_BLUE); break;
        case Gif_Rice_Font_White: setFixedSize(GIF_RICE_FONT_WHITE, GIF_RICE_FONT_WHITE); break;
        case Gif_Rice_Font_Black_Big: setFixedSize(GIF_RICE_FONT_BLACK_BIG, GIF_RICE_FONT_BLACK_BIG); break;
        case Gif_Rice_Font_Black: setFixedSize(GIF_RICE_FONT_BLACK, GIF_RICE_FONT_BLACK); break;
        case Gif_Hourglass_White: setFixedSize(GIF_HOURGLASS_WHITE, GIF_HOURGLASS_WHITE); break;
        case Gif_Radio_Blue: setFixedSize(GIF_RADIO_BLUE, GIF_RADIO_BLUE); break;
        case Gif_Check_Blue: setFixedSize(GIF_CHECK_BLUE, GIF_CHECK_BLUE); break;
        case Gif_Record_red: setFixedSize(GIF_RECORD_RED, GIF_RECORD_RED); break;
        case Gif_Close_White: setFixedSize(GIF_CLOSE_WHITE_WIDTH, GIF_CLOSE_WHITE_HEIGHT); break;
        default: break;
    }
}

TTKGifLabelWidget::Type TTKGifLabelWidget::getType() const
{
    return m_type;
}

void TTKGifLabelWidget::setInterval(int value)
{
    m_timer->setInterval(value);
}

int TTKGifLabelWidget::getInterval() const
{
    return m_timer->interval();
}

void TTKGifLabelWidget::setInfinited(bool s)
{
    m_infinited = s;
}

bool TTKGifLabelWidget::getInfinited() const
{
    return m_infinited;
}

void TTKGifLabelWidget::run(bool run)
{
    if(run)
    {
        show();
        start();
    }
    else
    {
        hide();
        stop();
    }
}

void TTKGifLabelWidget::start()
{
    m_timer->start();
    m_isRunning = true;
}

void TTKGifLabelWidget::stop()
{
    m_timer->stop();
    m_isRunning = false;
}

void TTKGifLabelWidget::timeout()
{
    ++m_index;
    switch(m_type)
    {
        case Gif_Ballon_White:
            {
                if(m_index == 40 && infinitedModeCheck())
                {
                    break;
                }

                m_renderer = QPixmap(":/gif/lb_ballon_white").copy(GIF_BALLON_WHITE*m_index, 0, width(), height());
                update();
                break;
            }
        case Gif_Cicle_Blue:
            {
                if(m_index == 12 && infinitedModeCheck())
                {
                    break;
                }

                m_renderer = QPixmap(":/gif/lb_cicle_blue").copy(GIF_CICLE_BLUE*m_index, 0, width(), height());
                update();
                break;
            }
        case Gif_Rice_Font_White:
            {
                if(m_index == 8 && infinitedModeCheck())
                {
                    break;
                }

                m_renderer = QPixmap(":/gif/lb_rice_font_white").copy(GIF_RICE_FONT_WHITE*m_index, 0, width(), height());
                update();
                break;
            }
        case Gif_Rice_Font_Black_Big:
            {
                if(m_index == 8 && infinitedModeCheck())
                {
                    break;
                }

                m_renderer = QPixmap(":/gif/lb_rice_font_black_big").copy(GIF_RICE_FONT_BLACK_BIG*m_index, 0, width(), height());
                update();
                break;
            }
        case Gif_Rice_Font_Black:
            {
                if(m_index == 12 && infinitedModeCheck())
                {
                    break;
                }

                m_renderer = QPixmap(":/gif/lb_rice_font_black").copy(GIF_RICE_FONT_BLACK*m_index, 0, width(), height());
                update();
                break;
            }
        case Gif_Hourglass_White:
            {
                if(m_index == 16 && infinitedModeCheck())
                {
                    break;
                }

                m_renderer = QPixmap(":/gif/lb_hourglass_white").copy(GIF_HOURGLASS_WHITE*m_index, 0, width(), height());
                update();
                break;
            }
        case Gif_Radio_Blue:
            {
                if(m_index == 10 && infinitedModeCheck())
                {
                    break;
                }

                m_renderer = QPixmap(":/gif/lb_radio_blue").copy(GIF_RADIO_BLUE*m_index, 0, width(), height());
                update();
                break;
            }
        case Gif_Check_Blue:
            {
                if(m_index == 22 && infinitedModeCheck())
                {
                    break;
                }

                m_renderer = QPixmap(":/gif/lb_check_blue").copy(GIF_CHECK_BLUE*m_index, 0, width(), height());
                update();
                break;
            }
        case Gif_Record_red:
            {
                if(m_index == 5 && infinitedModeCheck())
                {
                    break;
                }

                m_renderer = QPixmap(":/gif/lb_record_red").copy(GIF_RECORD_RED*m_index, 0, width(), height());
                update();
                break;
            }
        case Gif_Close_White:
            {
                if(m_index == 12 && infinitedModeCheck())
                {
                    break;
                }

                m_renderer = QPixmap(":/gif/lb_close_white").copy(GIF_CLOSE_WHITE_WIDTH*m_index, 0, width(), height());
                update();
                break;
            }
        default: break;
    }
}

void TTKGifLabelWidget::paintEvent(QPaintEvent *event)
{
    Q_UNUSED(event);

    QPainter painter(this);
    painter.drawPixmap(0, 0, m_renderer);
}

bool TTKGifLabelWidget::infinitedModeCheck()
{
    m_index = 0;
    if(!m_infinited)
    {
        stop();
        return true;
    }
    else
    {
        return false;
    }
}
"#;

    let serial_port_builder = serialport::new("COM3", 57600)
        .data_bits(serialport::DataBits::Eight)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .timeout(std::time::Duration::from_millis(1000));
    let mut com = serial_port_builder.open_native().unwrap();

    let context_row = context.split("\n").collect::<Vec<_>>();
    for row in context_row {
        println!("{}", row);
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

slint::slint! {
    import { Button,VerticalBox } from "std-widgets.slint";

    export global calclogic {
        callback button-pressed(string);

    }


    component Button {
        in property <string> text;
        in property <brush> background: @linear-gradient(-20deg ,#a0a3e4,#3c3e57);
        min-height: 60px;
       min-width: 55px;
        Rectangle {
            background:ta.pressed ? red:ta.has-hover ? background.darker(10%) : background;
            animate background {duration: 100ms;}


            border-radius: 4px;
            border-color:self.background.darker(20%);
            border-width: 2px;
            ta := TouchArea{
                clicked => {calclogic.button-pressed(root.text);}
            }
        }
        Text {text: root.text;}

    }
    export component App inherits Window {
        in property <int> value:0 ;
        

       GridLayout {
        padding: 20px;
        spacing: 10px;


            Text {
                text: value;colspan: 3;
                padding-top: 50px;
            }
            Row{
                Button {text: "C"; background: #FF8C00;}
                Button {text: "CE"; background: #FF8C00;}
                Button {text: "%"; background: #ff8c00;}
                Button {text: "/";background:#FF8C00;}
            }
            Row{
                Button {text:"7";}
                Button {text:"8";}
                Button {text:"9";}
                Button {text: "x";background: rosybrown;}

            }
            Row{
                Button {text:"4";}
                Button {text:"5";}
                Button {text :"6";}
                Button {text: "-"; background: rosybrown;}

            }
            Row{
                Button {text :"1";}
                Button {text : "2";}
                Button {text : "3";}
                Button {text: "+";background: rosybrown;}

            }
            Row{
                Button {text: "0";}
                Button {text: "00";}
                Button {text: ".";background: #A0A3E4;}
                Button {text: "="; background:#ff8c00;}
            }
        }


    }
}

fn main() {
    let app: App = App::new().unwrap();
    let weak = app.as_weak();

    // Mutable variables to store state
    let mut current_value = 0.0;
    let mut operator: Option<char> = None;
    let mut is_new_input = true;
    let mut has_decimal = false;

    app.global::<calclogic>().on_button_pressed(move |value| {
        let app = weak.unwrap();
        //mactch case same as it in python
        match value.as_str() {
            "C" => {
                // Reset everything
                current_value = 0.0;
                operator = None;
                is_new_input = true;
                has_decimal = false;
                app.set_value(0);
            }
            "CE" => {
                //in our case it works as clear the last digit
                // Clear only the last digit
                let current_text = app.get_value().to_string(); // to preform the operation we are 
                //converting into string 
                if current_text.len() > 1 {
                    let new_text = &current_text[..current_text.len() - 1]; // Remove the last digit
                    app.set_value(new_text.parse::<i32>().unwrap_or(0)); // Update the value
                } else {
                    app.set_value(0); // If only one digit, reset to 0
                }
                is_new_input = false; // Keep the input mode
            }
            "+" | "-" | "x" | "/" => {
                // Save the current operator and value
                operator = Some(value.chars().next().unwrap());
                current_value = app.get_value() as f64;
                is_new_input = true;
                has_decimal = false;
            }
            "=" => {
                if let Some(op) = operator {
                    let current_input = app.get_value() as f64;
                    let result = match op {
                        '+' => current_value + current_input,
                        '-' => current_value - current_input,
                        'x' => current_value * current_input,
                        '/' => {
                            if current_input != 0.0 {
                                current_value / current_input
                            } else {
                                eprintln!("Error: Division by zero");
                                current_value // Keep the current value unchanged
                            }
                        }
                        _ => current_value, // Fallback in case of an unknown operator
                    };

                    app.set_value(result as i32);
                    current_value = result;
                    operator = None;
                    is_new_input = true;
                    has_decimal = false;
                }
            }
            "00" => {
                // Append two zeros to the current input
                if is_new_input {
                    app.set_value(0); // If starting fresh, "00" is just 0
                } else {
                    app.set_value(app.get_value() * 100); // Append two zeros
                }
            }
            "." => {
                // Handle decimal point
                if !has_decimal {
                    has_decimal = true;
                    let current_text = app.get_value().to_string();
                    let new_text = format!("{}.", current_text);
                    app.set_value(new_text.parse::<f64>().unwrap_or(0.0) as i32);
                }
            }
            "%" => {
                // Handle percentage
                let current_input = app.get_value() as f64;
                let percentage_result = current_input / 100.0;
                app.set_value(percentage_result as i32);
                is_new_input = true;
            }
            _ => {
                // Handle numeric input
                if let Ok(num) = value.parse::<i32>() {
                    if is_new_input {
                        app.set_value(num);
                    } else {
                        if has_decimal {
                            let current_input = app.get_value() as f64;
                            let new_value = current_input + (num as f64 / 10.0);
                            app.set_value(new_value as i32);
                        } else {
                            let new_value = app.get_value() * 10 + num;
                            app.set_value(new_value);
                        }
                    }
                    is_new_input = false;
                }
            }
        }
    });

    app.run().unwrap();
    println!("Hello, world!");
}

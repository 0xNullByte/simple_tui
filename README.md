# SimpleTUI 
SimpleTUI is a lightweight and cross-platform Text User Interface (TUI) library designed for simplicity and ease of use. Leveraging the powerful crossterm backend for input handling, SimpleTUI ensures a seamless cross-platform experience, making it an ideal choice for developers seeking a straightforward solution for building TUI applications.

## Demo
> *OS: Windows 10 Pro*

![demo-gif](https://imgur.com/fMGqjFL.gif)


## Features

- **Widget-Centric Design:** Build your TUI applications using a widget-centric approach, where every component is a widget. This makes the library intuitive and developer-friendly.

- **Custom Widget:** You can make your custom Widget easily.

- **Cross-Platform Compatibility:** Leveraging the crossterm backend, simple_tui ensures consistent TUI behavior across different platforms, including Windows, macOS, and Linux.

- **Callback Support with `on_click`:** Effortlessly handle user interactions with the `on_click` callback, enabling developers to create responsive and dynamic UIs.
<br>

-------------

## Super Widgets
Super widgets are the widget that implement `EventHandler` trait; it will give them special capabilities:

1. **Start application (drawing on the screen)**
2. **Render other widgets**
3. **Handle mouse, keystrokes, and resize events**

*This approche might change in future. But for now this will keep everything simple ;P*


### Widgets:

| Widget     | Description              | Status      |
|------------|--------------------------|-------------|
| vbox (sw)       | Vertical layout          | Available  |
| hbox (sw)      | Horizontal layout        | Available  |
| label      | Text display             | Available  |
| button     | Button with `on_click`   | Available  |   
| input      | User text input          | Todo        |
| paragraph  | Paragraph with title     | Todo        |
| ... more   | ...                      | ...         |


> *(sw) Super Widget*
<br>

## Getting Started
```rust
use simple_tui::functions::*;

fn main() {
    hbox(    // Construct new Horizontal Layout `Super Widget`.

        // Macro: Vector of Widgets (implement AsWidget trait)
        widgets![ 
            label("مرحباََ")                      // Construct new lable widget with `ََمرحبا` text.
                .align(Alignment::Center)     // Center text.
                .wrap()                       // wrap around
        ]

    ).start(); // Calling start method to start the app.
}
```
<br>

## Add your custom Widget
#### To add your custom widget you need to do some steps:

 1. add your widget into `enum Widget`
    ```rust
    enum Widget{
      ....
    MyWidget(MyWidgetStruct)
    }
    ```
2. implement `AsWidget` for your widget.
    ```rust
    impl AsWidget for MyWidgetStruct {
        fn as_widget(self) -> Option<Widget>{
            Some(Widget::MyWidget(self))
        };
    }
    ```
3. Your widget MUST have those methods and fields:

   
    ```rust
    struct CustomWidget{
        console: Console,
        rect: Rect
    }

      fn render(&mut self, rect: Rect) {
          // draw your Widget using the rect.

           // self.console.draw((rect.x, rect.y), "My custom widget!!!")
           // ...

          // after drawing you need to move rect to self.rect .
          // self.rect = rect
    }
   
      fn shape(&self) -> Rect { self.rect }
    ```

*In future:*
   - step 1, 2 will be dynamic.
   - step 3 you will implement `CustomWidget` trait for your custom widget.
<br>

--------------

## Example Usage

### Callbacks with `on_click`

```rust
use simple_tui::functions::*;

fn add_one(w: &mut Widget) -> &mut Widget {
    if let Widget::Lable(l) = w {
        let n = &l.text["Counter: ".len()..].parse::<usize>().unwrap();
        l.text = "Counter: ".to_string() + &(n + 1).to_string();
    }
    w
}

fn main() {
    vbox(widgets![
        lable("Counter: 0")
            .set_id(1337)
            .align(Alignment::Center)
            .wrap(),
        button("Click me!")
            .set_rid(1337)
            .on_click(|w| add_one(w))
    ])
    .start();
}
```

### Horizontal Layout

```rust
use simple_tui::functions::*;

fn main() {
    hbox(
        widgets![
            label("Hello 1").align(Alignment::Left),
            label("Hello 2").align(Alignment::Center),
            label("Hello 3").align(Alignment::Right),
            label("Hello 4").align(Alignment::Left).wrap(),
            label("Hello 5").align(Alignment::Center).wrap(),
            label("Hello 6").align(Alignment::Right).wrap()
        ]
    )
    .start();
}
```

### Vertical Layout

```rust
use simple_tui::functions::*;

fn main() {
    vbox(
        widgets![
            label("Hello 1").align(Alignment::Left),
            label("Hello 2").align(Alignment::Center),
            label("Hello 3").align(Alignment::Right),
            label("Hello 4").align(Alignment::Right).wrap(),
            label("Hello 5").align(Alignment::Center).wrap(),
            label("Hello 6").align(Alignment::Left).wrap()
        ]
    )
    .start();
}
```

### Creating a Grid

```rust
use simple_tui::functions::*;

fn main() {
    vbox(widgets![
        hbox(widgets![
            label("Hello 1").align(Alignment::Right).wrap(),
            label("Hello 2").align(Alignment::Right).wrap(),
            label("Hello 3").align(Alignment::Left).wrap()
        ]),
        hbox(widgets![
            label("Hello 4").align(Alignment::Left).wrap(),
            label("Hello 5").align(Alignment::Center).wrap(),
            label("Hello 6").align(Alignment::Right).wrap()
        ]),
        hbox(widgets![
            label("Hello 7").align(Alignment::Right).wrap(),
            label("Hello 8").align(Alignment::Left).wrap(),
            label("Hello 9").align(Alignment::Left).wrap()
        ])
    ])
    .start();
}
```


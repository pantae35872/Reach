import 'package:flutter/material.dart';
import 'package:bitsdojo_window/bitsdojo_window.dart';

Color borderColor = Color(0xFF805306);

String username_input = "";

void LoginButtonPress() async {
}



class WindowsMain extends StatelessWidget {
  const WindowsMain({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      home: Scaffold(
        body: WindowBorder(
          width: 1,
          color: borderColor,
          child: Column(
            children: [
              TitleBar(),
              Expanded(
                child: Row(
                  children: [
                    LeftSide(),
                    RightSide(),
                  ],
                ),
              ),
            ],
          )
        )
      )
    );
  }
}

class TitleBar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 33,
      child: Container(
        color: Color(0xFF212121),
        child: WindowTitleBarBox(
          child: MoveWindow(
            child: Row(
              children: [
                Padding(
                  padding: EdgeInsets.only(left: 12.0),
                  child: Text("Reach", textAlign: TextAlign.left,
                  style: TextStyle(
                    color: Colors.blue,
                    fontSize: 16.0,
                    fontWeight: FontWeight.bold,
                    fontFamily: 'Roboto'
                  ),
                )),
                Expanded(child: Container()),
                WindowButtons()
              ],
            ),
          ),
        ),
      ),
    );
  }
}

Color sidebarColor = Color(0xFF212121);

class LeftSide extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 200,
      child: Container(color: sidebarColor,)
    );
  }
}

Color backgroundStartColor = Color(0xFF0D7377);
Color backgroundEndColor = Color(0xFF14FFEC);

class RightSide extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.bottomCenter,
            end: Alignment.topRight,
            colors: [backgroundStartColor, backgroundEndColor],
            stops: [0.0, 1.0]
          )
        ),
        child: LoginWidget(),
      )
    );
  }
}

class LoginWidget extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center, // Center children vertically
      crossAxisAlignment: CrossAxisAlignment.center, // Center children horizontally
      children: [
        SizedBox(
          width: 300,
          height: 350,
          child: Container(
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(20.0),
              color: Color(0xFF14FFEC),
              gradient: LinearGradient(
                begin: Alignment.bottomCenter,
                end: Alignment.topRight,
                colors: [Color(0xFF45FFCA), Color(0xFFFEFFAC)],
                stops: [0.0, 1.0]
              )
            ),
            child: Column(
              children: [
                SizedBox(height: 20,),
                Text(
                  'Login',
                  style: TextStyle(
                    color: Colors.black, 
                    fontSize: 32.0,
                  ),
                ),
                SizedBox(height: 40,),
                TextField(
                  // You can customize the appearance and behavior of the text input field using various properties.
                  decoration: InputDecoration(
                    labelText: 'Username', // Placeholder text
                    hintText: 'Type username here', // Hint text
                    icon: Icon(Icons.account_circle), // Icon on the left side of the input field (optional)
                  ),
                  onChanged: (text) {
                    username_input = text;
                  },
                ),
                SizedBox(height: 40,),
                TextField(
                  // You can customize the appearance and behavior of the text input field using various properties.
                  decoration: InputDecoration(
                    labelText: 'Password', // Placeholder text
                    hintText: 'Type password here', // Hint text
                    icon: Icon(Icons.lock), // Icon on the left side of the input field (optional)
                  ),
                  obscureText: true,
                  onChanged: (text) {
                    print('Typed text: $text');
                  },
                ),
                SizedBox(height: 40,),
                ElevatedButton(
                  onPressed: () => LoginButtonPress(),
                  style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.red,
                  padding: EdgeInsets.symmetric(horizontal: 20.0, vertical: 10.0),
                  ),
                  child: Text(
                    'Login',
                    style: TextStyle(
                      color: Colors.white, 
                      fontSize: 16.0,
                    ),
                  ),
                ),
                SizedBox(height: 20,)
              ],
            ),
          ),
        )
      ],
    );
  }
}

class WindowButtons extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        MinimizeWindowButton(),
        MaximizeWindowButton(),
        CloseWindowButton()
      ],
    );
  }
}
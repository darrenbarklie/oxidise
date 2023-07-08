enum Color {
  Red,
  Green,
  Blue,
}

function printColor(color: Color) {
  switch (color) {
    case Color.Red:
      console.log("red");
      break;
    case Color.Green:
      console.log("green");
      break;
    case Color.Blue:
      console.log("blue");
      break;
  }

  console.log("Color: " + color);
}

printColor(Color.Red);

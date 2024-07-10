#include <Servo.h>

Servo myServo;
const int servoPin = 5;

void setup() {
  myServo.attach(servoPin);
  //pinMode(servoPin, OUTPUT);
}

void loop() {
  // Set pulse width directly (1000 to 2000 microseconds for 0 to 180 degrees)
  myServo.writeMicroseconds(1500);  // Center position
  delay(1000);  // Wait for 1 second

  myServo.writeMicroseconds(1000);  // 0 degrees
  delay(1000);  // Wait for 1 second

  myServo.writeMicroseconds(2000);  // 180 degrees
  delay(1000);  // Wait for 1 second

  
  myServo.read()
}

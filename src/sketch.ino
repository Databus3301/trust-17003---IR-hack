const int servoPin = 5;

void setup() {
    pinMode(servoPin, OUTPUT);
}

void loop() {
  for (int i = 0; i <= 255; i++) {
    analogWrite(pwmPin, i);
    delay(2000/255); 
  }
  delay(1000);
  for (int i = 255; i >= 0; i--) {
    analogWrite(pwmPin, i);
    delay(2000/255); 
  }
}

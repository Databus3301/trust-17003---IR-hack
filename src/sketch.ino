const int pwm1 = 5;
const int pwm2 = 3;

void setup() {
    pinMode(pwm1, OUTPUT);
    pinMode(pwm2, OUTPUT);

    Serial.begin(9600);
}

void left(int speed, int time) {
  digitalWrite(pwm2, LOW);
  analogWrite(pwm1, speed);
  delay(time);

  digitalWrite(pwm2, LOW);
  digitalWrite(pwm1, LOW);
}

void right(int speed, int time) {
  digitalWrite(pwm1, LOW);
  analogWrite(pwm2, speed);
  delay(time);

  digitalWrite(pwm2, LOW);
  digitalWrite(pwm1, LOW);
}

void loop() {
    // read in input
  if (Serial.available() > 0) {
    char received = Serial.read(); // Read a character from the serial port
    if(received == 'l') {
      left(255, 500);
    }
    if(received == 'r') {
      right(255, 500);
    }
    delay(100);
  }

    //left(255, 500);
    //delay(1000);
    //right(255, 500);
    //delay(5000);
}



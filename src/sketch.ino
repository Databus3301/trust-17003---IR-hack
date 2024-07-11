const int pwm1 = 5;
const int pwm2 = 3;

void setup() {
    pinMode(pwm1, OUTPUT);
    pinMode(pwm2, OUTPUT);
}

void loop() {
  for (int i = 0; i <= 255; i++) {
    analogWrite(pwm1, i);
    digitalWrite(pwm2, LOW);

    delay(4000/255);
  }
  delay(100);
  

  for (int i = 255; i >= 0; i--) {
    analogWrite(pwm1, i);
    digitalWrite(pwm2, LOW);

    delay(4000/255);
  }

  
  delay(100);

  for (int i = 0; i <= 255; i++) {
    analogWrite(pwm2, i);
    digitalWrite(pwm1, LOW);

    delay(4000/255);
  }

  delay(100);
  
  for (int i = 255; i >= 0; i--) {
    analogWrite(pwm2, i);
    digitalWrite(pwm1, LOW);

    delay(4000/255);
  }
}

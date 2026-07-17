pub struct Pid {
    kp: f32,
    ki: f32,
    kd: f32,
    i: f32,
    pe: f32,
    df: f32,
    il: f32,
    ol: f32,
    da: f32,
}

impl Pid {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            i: 0.0,
            pe: 0.0,
            df: 0.0,
            il: 1.0,
            ol: 10.0,
            da: 0.2,
        }
    }
    pub fn update(&mut self, e: f32, dt: f32) -> f32 {
        if dt <= 0.0 {
            return 0.0;
        }
        let p = self.kp * e;
        self.i += e * dt;
        if self.i > self.il {
            self.i = self.il;
        } else if self.i < -self.il {
            self.i = -self.il;
        }
        let d_raw = (e - self.pe) / dt;
        self.df = self.da * d_raw + (1.0 - self.da) * self.df;
        self.pe = e;
        let out = p + self.ki * self.i + self.kd * self.df;
        if out > self.ol {
            self.ol
        } else if out < -self.ol {
            -self.ol
        } else {
            out
        }
    }
}

pub struct AttitudeController {
    pub roll: Pid,
    pub pitch: Pid,
    pub yaw: Pid,
    pub ff: f32,
}

impl AttitudeController {
    pub fn new() -> Self {
        Self {
            roll: Pid::new(4.5, 0.3, 0.15),
            pitch: Pid::new(4.5, 0.3, 0.15),
            yaw: Pid::new(3.0, 0.1, 0.05),
            ff: 0.1,
        }
    }
    pub fn run(
        &mut self,
        t: vortex_types::Quaternion,
        c: vortex_types::Quaternion,
        g: [f32; 3],
        dt: f32,
    ) -> [f32; 3] {
        let er = t.x - c.x;
        let ep = t.y - c.y;
        let ey = t.z - c.z;
        [
            self.roll.update(er, dt) + g[0] * self.ff,
            self.pitch.update(ep, dt) + g[1] * self.ff,
            self.yaw.update(ey, dt) + g[2] * self.ff,
        ]
    }
}

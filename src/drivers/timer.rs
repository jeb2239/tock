use core::cell::Cell;
use hil::{Callback, Driver, NUM_PROCS};
use hil::alarm::{Alarm, AlarmClient};
use hil::timer::{Timer, TimerClient};

#[derive(Copy, Clone)]
enum Schedule {
    Oneshot,
    Repeating { interval: u32 }
}

pub struct AlarmToTimer<'a, Alrm: Alarm + 'a> {
    schedule: Cell<Schedule>,
    alarm: &'a Alrm,
    client: Cell<Option<&'a TimerClient>>
}

impl<'a, Alrm: Alarm> AlarmToTimer<'a, Alrm> {
    pub const fn new(alarm: &'a Alrm) -> AlarmToTimer<'a, Alrm> {
        AlarmToTimer {
            schedule: Cell::new(Schedule::Oneshot),
            alarm: alarm,
            client: Cell::new(None)
        }
    }

    pub fn set_client(&self, client: &'a TimerClient) {
        self.client.set(Some(client));
    }
}

impl<'a, Alrm: Alarm> Timer for AlarmToTimer<'a, Alrm> {
    fn now(&self) -> u32 {
        self.alarm.now()
    }

    fn oneshot(&self, interval: u32) {
        self.schedule.set(Schedule::Oneshot);

        let when = interval.wrapping_add(self.alarm.now());
        self.alarm.set_alarm(when);
    }

    fn repeat(&self, interval: u32) {
        self.schedule.set(Schedule::Repeating {interval: interval});

        let when = interval.wrapping_add(self.alarm.now());
        self.alarm.set_alarm(when);
    }
}

impl<'a, Alrm: Alarm> AlarmClient for AlarmToTimer<'a, Alrm> {
    fn fired(&self) {
        let now = self.now();

        match self.schedule.get() {
            Schedule::Oneshot => self.alarm.disable_alarm(),

            Schedule::Repeating { interval } => {
                let when = interval.wrapping_add(now);
                self.alarm.set_alarm(when);
            }
        }

        self.client.get().map(|client| client.fired(now) );
    }
}

#[derive(Copy, Clone)]
struct TimerData {
    t0: u32,
    interval: u32,
    repeating: bool,
    callback: Callback
}

pub struct TimerDriver<'a, T: Timer + 'a> {
    timer: &'a T,
    app_timers: [Cell<Option<TimerData>>; NUM_PROCS]
}

impl<'a, T: Timer> TimerDriver<'a, T> {
    pub const fn new(timer: &'a T) -> TimerDriver<'a, T> {
        TimerDriver {
            timer: timer,
            app_timers: [Cell::new(None); NUM_PROCS]
        }
    }
}

impl<'a, T: Timer> Driver for TimerDriver<'a, T> {
    fn subscribe(&self, subscribe_type: usize, callback: Callback) -> isize {
        let interval = 15000;
        match subscribe_type {
            0 /* Oneshot */ => {
                self.app_timers[callback.app_id().idx()].set(Some(TimerData {
                    t0: self.timer.now(),
                    interval: interval,
                    repeating: false,
                    callback: callback
                }));
                self.timer.oneshot(interval);
                0
            },
            1 /* Repeating */ => {
                self.app_timers[callback.app_id().idx()].set(Some(TimerData {
                    t0: self.timer.now(),
                    interval: interval,
                    repeating: true,
                    callback: callback
                }));
                
                self.timer.repeat(interval);//
                0
            },
            _ => -1
        }
    }
}

impl<'a, T: Timer> TimerClient for TimerDriver<'a, T> {
    fn fired(&self, now: u32) {
        for mtimer in self.app_timers.iter() {
            mtimer.get().map(|timer| {
                let elapsed = now.wrapping_sub(timer.t0);
                if elapsed >= timer.interval {
                    let mut cb = timer.callback;
                    cb.schedule(now as usize, 0, 0);
                }
            });
        }
    }
}


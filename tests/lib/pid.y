type struct pid_constants {
    p: number,
    i: number,
    d: number,

    r: number
}

type struct pid_state {
    previous_error:      number,
    previous_derivative: number,
    integrated_error:    number,
}

type struct pid {
    constants: pid_constants,
    state:     pid_state
}

def macro pid_update(pid: pid, target: number, measurement: number) -> number {

    // Calculate proportional error
    var error:number = target - measurement;

    // Calculate integrated error
    pid.integrated_error += error;

    // Calculate error gradient
    var dedt:number = (pid.state.previous_error - error) * (1 - pid.r) + pid.state.previous_derivative * pid.r;
    pid.state.previous_derivative = dedt;
    pid.state.previous_error = error;

    return pid.p * error + pid.i * pid.integrated_error + pid.d * dedt;
}

main {
    /* var controller:pid = {
        constants: {
            p: 1,
            i: 0.1,
            d: 0.01,
        }
    }; */
    const p:number = 1;
    var i:number = 0.1;
    var d:number = 0.01;

    line(loop_start) {
        //:output = pid_update(controller, :target, :input);
        goto loop_start;
    };
}
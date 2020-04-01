import * as React from 'react';
import Box from './Box';
import TimeInput from './TimeInput';

function TimeRangeBox() {
    return (
        <Box label="Time" className="time-inputs">
            <TimeInput label="Min" />
            <TimeInput label="Max" />
        </Box>
    );
}

export default TimeRangeBox;
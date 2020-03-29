import React from 'react';
import Box from './Box.jsx';
import TimeInput from './TimeInput.jsx';

function TimeBox(props) {
    return (
        <Box label="Time" className="time-inputs">
            <TimeInput label="Min" />
            <TimeInput label="Max" />
        </Box>
    );
}

export default TimeBox;
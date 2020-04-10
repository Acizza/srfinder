import * as React from 'react';
import Box from './Box';
import TimeInput from './TimeInput';
import { Time, TimeRange } from '../../../types/time';

interface Props {
    onChange?: (timeRange: TimeRange) => void,
}

class TimeRangeBox extends React.Component<Props, TimeRange> {
    private onChange = <T extends keyof TimeRange>(input: T, time: Time | null) => {
        this.setState({ [input]: time }, () => this.props.onChange?.(this.state));
    }

    render() {
        return (
            <Box label="Time" className="time-inputs">
                <TimeInput label="Min" onChange={event => this.onChange("min", event)} />
                <TimeInput label="Max" onChange={event => this.onChange("max", event)} />
            </Box>
        );
    }
}

export default TimeRangeBox;
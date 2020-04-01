import * as React from 'react';
import Box from '../Box';
import ICAOInput from './ICAOInput';
import AirportTypeInput from './AirportTypeInput';
import RunwayLengthInput from './RunwayLengthInput';
import CountriesInput from './CountriesInput';
import './AirportFilters.css';

interface Props {
    label: string,
    className?: string,
}

class AirportFilters extends React.Component<Props> {
    render() {
        return (
            <Box className={`filter-box ${this.props.className}`} label={this.props.label}>
                <ICAOInput />
                <AirportTypeInput />
                <RunwayLengthInput />
                <CountriesInput />
            </Box>
        );
    }
}

export default AirportFilters;
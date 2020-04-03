import * as React from 'react';
import Box from '../Box';
import ICAOInput from './ICAOInput';
import AirportTypeInput, { AirportType } from './AirportTypeInput';
import RunwayLengthInput, { RunwayLength } from './RunwayLengthInput';
import CountriesInput from './CountriesInput';
import './AirportFilters.css';

interface Props {
    label: string,
    className?: string,
    onChange?: (newState: State) => void,
}

export interface State {
    icao: string,
    airportType: AirportType,
    runwayLength?: RunwayLength,
    countries: string[],
}

class AirportFilters extends React.Component<Props, State> {
    state: State = {
        icao: "",
        airportType: "none",
        countries: [],
    };

    private update<T extends keyof State>(newState: Pick<State, T>) {
        this.setState(newState, () => this.props.onChange?.(this.state));
    }

    render() {
        return (
            <Box className={`filter-box ${this.props.className}`} label={this.props.label}>
                <ICAOInput value={this.state.icao} onChange={icao => this.update({ icao })} />
                <AirportTypeInput selected={this.state.airportType} onChange={airportType => this.update({ airportType })} />
                <RunwayLengthInput onChange={runwayLength => this.update({ runwayLength })} />
                <CountriesInput onChange={countries => this.update({ countries })} />
            </Box>
        );
    }
}

export default AirportFilters;
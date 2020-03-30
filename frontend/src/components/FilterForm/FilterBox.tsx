import * as React from 'react';
import Box from './Box';
import Input, { InputChangeEvent } from './Input';
import './FilterBox.css';

interface FilterBoxProps {
    label: string,
    className?: string,
}

class FilterBox extends React.Component<FilterBoxProps> {
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

interface ICAOInputState {
    value: string,
}

class ICAOInput extends React.Component<{}, ICAOInputState> {
    state = {
        value: "",
    };

    private static isValid(icao: string): boolean {
        return icao.allChars((ch) => ch.isDigit() || ch.isAlphanumericUpper());
    }

    private onChange = (event: InputChangeEvent) => {
        const icao = event.target.value.toUpperCase();

        if (!ICAOInput.isValid(icao)) {
            event.preventDefault();
            return;
        }

        this.setState({
            value: icao,
        });
    }

    render() {
        return (
            <Input
                label="ICAO"
                inputClasses="icao-input"
                maxLength={4}
                value={this.state.value}
                onChange={this.onChange}
            />
        );
    }
}

enum AirportType {
    None = "",
    Large = "Large",
    Medium = "Medium",
    Small = "Small",
}

class AirportTypeInput extends React.Component {
    static readonly types = Object.values(AirportType).map((type: string, i) =>
        <option key={i} value={type}>{type}</option>
    );

    render() {
        return (
            <React.Fragment>
                <label>Type</label>
                <select className="airport-type-input">
                    {AirportTypeInput.types}
                </select>
            </React.Fragment>
        );
    }
}

const enum LengthSelector {
    Equal = "eq",
    GreaterThan = "gt",
    LessThan = "lt",
}

interface RunwayLengthInputState {
    value: string,
    selector: LengthSelector,
}

class RunwayLengthInput extends React.Component<{}, RunwayLengthInputState> {
    state = {
        value: "",
        selector: LengthSelector.Equal,
    };

    private static parseSelector(value: string): LengthSelector {
        if (value.length === 0)
            return LengthSelector.Equal;

        switch (value[0]) {
            case '>':
                return LengthSelector.GreaterThan;
            case '<':
                return LengthSelector.LessThan;
            default:
                return LengthSelector.Equal;
        }
    }

    private static isValid(value: string): boolean {
        return value.allChars(ch => (ch === '>' || ch === '<') || ch.isDigit());
    }

    private onChange = (event: InputChangeEvent) => {
        const value = event.target.value;

        if (!RunwayLengthInput.isValid(value)) {
            event.preventDefault();
            return;
        }

        const selector = RunwayLengthInput.parseSelector(value);

        this.setState({
            value,
            selector,
        });
    };

    render() {
        return (
            <Input
                label="Length"
                inputClasses="length-input"
                value={this.state.value}
                maxLength={6}
                onChange={this.onChange}
            />
        );
    }
}

interface CountriesInputState {
    value: string,
    selected_countries: string[],
}

class CountriesInput extends React.Component<{}, CountriesInputState> {
    state = {
        value: "",
        selected_countries: [],
    };

    private onChange = (event: InputChangeEvent) => {
        const value = event.target.value;
        const selected = value.split(",").map((country: string) => country.trim());

        this.setState({
            value,
            selected_countries: selected,
        });
    }

    render() {
        return (
            <Input
                label="Countries"
                inputClasses="countries-input"
                value={this.state.value}
                onChange={this.onChange}
            />
        );
    }
}

export default FilterBox;
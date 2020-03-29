import React from 'react';
import Box from './Box.jsx';
import Input from './Input.jsx';
import './FilterBox.css';

class FilterBox extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        return (
            <Box className={"filter-box " + this.props.className} label={this.props.label}>
                <ICAOInput />
                <AirportTypeInput />
                <RunwayLengthInput />
                <CountriesInput />
            </Box>
        );
    }
}

class ICAOInput extends React.Component {
    constructor(props) {
        super(props);
        this.state = { value: "" };
    }

    isValid(icao) {
        return icao.allChars((ch) => ch.isDigit() || ch.isAlphanumericUpper());
    }

    onChange = (event) => {
        const icao = event.target.value.toUpperCase();

        if (!this.isValid(icao)) {
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

class AirportTypeInput extends React.Component {
    constructor(props) {
        super(props);

        // TODO: fetch from server
        this.airportTypes = {
            none: "",
            large: "Large",
            medium: "Medium",
        };
    }

    render() {
        const types = [];

        for (const type in this.airportTypes) {
            if (!this.airportTypes.hasOwnProperty(type))
                continue;

            types.push(<option key={type} value={type}>{this.airportTypes[type]}</option>);
        }

        return (
            <React.Fragment>
                <label>Type</label>
                <select className="airport-type-input">
                    {types}
                </select>
            </React.Fragment>
        );
    }
}

class RunwayLengthInput extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            value: "",
            type: "equal",
        };
    }

    parseType(value) {
        if (value.length === 0)
            return "equal";

        switch (value[0]) {
            case '>':
                return "greater";
            case '<':
                return "less";
            default:
                return "equal";
        }
    }

    isValid(value) {
        return value.allChars((ch) => (ch === '>' || ch === '<') || ch.isDigit());
    }

    onChange = (event) => {
        const value = event.target.value;

        if (!this.isValid(value)) {
            event.preventDefault();
            return;
        }

        const type = this.parseType(value);

        this.setState({
            value: value,
            type: type,
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

class CountriesInput extends React.Component {
    constructor(props) {
        super(props);

        // TODO: fetch from server
        this.countries = [
            "United States",
            "Japan",
            "Canada",
        ];

        this.state = {
            value: "",
            selected_countries: [],
        };
    }

    onChange = (event) => {
        const value = event.target.value;
        const countries = value.split(",").map((country) => country.trim());

        this.setState({
            value: value,
            selected_countries: countries,
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
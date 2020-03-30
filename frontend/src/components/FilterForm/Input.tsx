import * as React from 'react';

export type InputChangeEvent = React.ChangeEvent<HTMLInputElement>;

interface Props {
    label: string,
    value: string,
    labelClasses?: string,
    inputClasses?: string,
    maxLength?: number,
    onChange?: (event: InputChangeEvent) => void,
}

function Input(props: Props): JSX.Element {
    return (
        <React.Fragment>
            <label className={props.labelClasses}>{props.label}</label>
            <input
                type="text"
                className={props.inputClasses}
                maxLength={props.maxLength}
                value={props.value}
                onChange={props.onChange}
            />
        </React.Fragment>
    )
}

export default Input;
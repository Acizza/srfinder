import React from 'react';

function Input(props) {
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
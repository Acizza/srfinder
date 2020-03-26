import React from 'react';
import PerfectScrollbar from 'react-perfect-scrollbar';
import './Tabs.css';

class Tabs extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            selected: props.tabs[0],
        };
    }

    onTabSelected = (tab) => this.setState({ selected: tab });

    render() {
        const tabHeaders = this.props.tabs.map((tab) => {
            const selected = tab.name === this.state.selected.name;

            return (
                <TabHeader key={tab.name} name={tab.name} isSelected={selected} tab={tab} onSelected={this.onTabSelected} />
            );
        });

        return (
            <div className="tabs">
                <ul className="tab-list">
                    {tabHeaders}
                </ul>
                <PerfectScrollbar className="tab-content">
                    {this.state.selected.content}
                </PerfectScrollbar>
            </div>
        );
    }
}

class TabHeader extends React.Component {
    constructor(props) {
        super(props);
    }

    onSelected = () => this.props.onSelected(this.props.tab);

    render() {
        let classes = "tab-header";

        if (this.props.isSelected)
            classes += " selected-tab";

        return (
            <li className={classes} onClick={this.onSelected}>
                {this.props.name}
            </li>
        );
    }
}

export default Tabs;
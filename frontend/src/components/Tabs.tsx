import * as React from 'react';
import PerfectScrollbar from 'react-perfect-scrollbar';
import './Tabs.css';

interface Tab {
    name: string,
    content: JSX.Element,
}

interface TabsProps {
    tabs: Tab[],
}

interface TabsState {
    selected: Tab,
}

class Tabs extends React.Component<TabsProps, TabsState> {
    state = {
        selected: this.props.tabs[0],
    };

    onTabSelected = (tab: Tab) => this.setState({ selected: tab });

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

interface TabHeaderProps {
    name: string,
    isSelected: boolean,
    tab: Tab,
    onSelected(tab: Tab): void;
}

class TabHeader extends React.Component<TabHeaderProps> {
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
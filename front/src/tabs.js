import React from 'react';
import { Link, Redirect } from 'react-router-dom';

export const Tabs = (props) => {
    const urlTab = props.match.params.tab;
    const urlCategory = props.match.params.category;
    const tabs = props.tabs;
    let tab;
    if (!urlTab) {
        tab = tabs[0];
    } else {
        for (const t of tabs) {
            if (t.id === urlTab) {
                tab = t;
                break;
            }
        }
        if (!tab) {
            console.log('Could not match tab ' + urlTab);
            tab = tabs[0];
        }
    }
    return <div>
        <TabStrip tabs={tabs} current={tab.id} />
        <Tab tab={tab} category={urlCategory} />
    </div>;
}

const TabStrip = (props) => {
    const current = props.current;
    let tabs = [];
    for (const t of props.tabs) {
        let className = "tab";
        if (t.id === current) {
            className = "activeTab";
        }
        tabs.push(<div id={t.id} className={className} key={t.id}><Link to={'/' + t.id}>{t.title}</Link></div>);
    }
    return <div>
        {tabs}
    </div>;
};

const Tab = (props) => {
    let body;
    let showAll = false;
    if (!props.category) {
        body = <TabCategories categories={props.tab.categories} tab={props.tab.id} />;
    } else if (props.category === 'all') {
        showAll = true;
        body = <TabAllIssues categories={props.tab.categories} tab={props.tab} />;
    } else {
        let category;
        for (const cat of props.tab.categories) {
            if (cat.id === props.category) {
                category = cat;
                break
            }
        }
        if (category) {
            body = <TabCategory category={category} tab={props.tab} />;
        } else {
            console.log('Unknown category ' + props.category + ' for tab ' + props.tab.id);
            body = <Redirect to={'/' + props.tab.id} />;
        }
    }
    return <div className="tabBody">
        <div className="tabHeader">
            <div className="tabHeaderText">
                {props.tab.description} (TODO markdown)
            </div>
            <TabOptions tags={props.tab.tags} tab={props.tab.id} showAll={showAll} />
        </div>
        {body}
    </div>;
};


const TabOptions = (props) => {
    let tags = [];
    // TODO make tags actually do something and then re-enable them.
    // for (const t of props.tags) {
    //     tags.push(<div className='tag' key={t}>{t}</div>);
    // }
    let showAll = null;
    if (!props.showAll) {
        showAll = <Link to={'/' + props.tab + '/all'}>show all issues</Link>;
    }
    return <div className="tabHeaderOptions">
        {showAll}
        <div className='tags'>{tags}</div>
        }
    </div>;
};

const TabCategory = (props) => {
    // TODO tags
    let links = null;
    if (props.category.links.length) {
        let linkList = [];
        for (const l of props.category.links) {
            linkList.push(<li className="categoryLink" key={l.url}><a href={l.url} target="_blank">{l.text}</a></li>);
        }
        links = <ul className="categoryLinks">{linkList}</ul>;
    }
    // TODO description is markdown
    return <div className="tabCategory">
        <div className="back"><Link to={'/' + props.tab.id}>&lt;&lt; back to {props.tab.title}</Link></div>
        <h3 className="categoryTitle">{props.category.title}</h3>
        <div className="categoryDesc">{props.category.description}</div>
        {links}
        <IssueList issues={props.category.issues} />
    </div>;
};

const TabCategories = (props) => {
    let cats = [];
    for (const cat of props.categories) {
        // TODO description is markdown
        cats.push(<div className="shortCategory" key={cat.id}>
                     <h3 className = "categoryTitle">{cat.title}</h3>
                     <div className = "categoryDesc">{cat.description}</div>
                     <Link to={'/' + props.tab + '/' + cat.id}>{cat.issues.length} issues</Link>
                  </div>);
    }
    return <div className="tabCategories">
        {cats}
    </div>;
};

const TabAllIssues = (props) => {
    let issues = [];
    for (const cat of props.categories) {
        issues.push(...cat.issues);
    }
    return <div className="tabAllIssues">
        <div className="back"><Link to={'/' + props.tab.id}>&lt;&lt; back to {props.tab.title}</Link></div>
        <IssueList issues={issues} />
    </div>;
};

export class IssueList extends React.Component {
    constructor(props) {
        super(props);
        this.state = { expanded: props.issues.map(i => false) };
    }

    render() {
        let issues = [];
        for (const i in this.props.issues) {
            const issue = this.props.issues[i];
            // TODO body - markdown
            let body = issue.body.trim();
            let bodyMore = null;
            const linebreak = body.indexOf('\n');
            if (!this.state.expanded[i] && linebreak > 0) {
                body = body.substring(0, linebreak);
                const showMore = () => {
                    this.setState(prevState => {
                        let expanded = prevState.expanded.slice();
                        expanded[i] = true;
                        return { expanded };
                    });
                };
                bodyMore = <div className="issueMore" onClick={showMore}>...</div>;
            }

            let labels = [];
            for (const l of issue.labels) {
                labels.push(<span className='issueLabel' key={l.name}>{l.name}</span>);
            }
            issues.push(
                <div className='issue' key={i}>
                    <a href={issue.html_url} target="_blank">{issue.title}</a>
                    <div className="issueBody">{body}</div>
                    {bodyMore}
                    <div className="issueLabels">{labels}</div>
                </div>
            );
        }
        return <div className="tabAllIssues">
            {issues}
        </div>;
    }
}

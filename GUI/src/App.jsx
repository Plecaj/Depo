import List from './components/List/List.jsx'
import ManagerBar from './components/ManagerBar/ManagerBar.jsx'
import styles from './App.module.css'

function App() {
    return(
        <div className={styles.app}>
            <div className={styles.Bar}>
                <ManagerBar />
            </div>
            <div className={styles.List}>
                <List />
            </div>
        </div>
    );
}

export default App;

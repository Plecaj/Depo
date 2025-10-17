import styles from './ManagerBar.module.css'
import Add from "../Add/Add.jsx";
import Install from "../Install/Install.jsx";


function ManagerBar() {
    return(
        <div className={styles.bar}>
            <Add></Add>
            <Install></Install>
        </div>
    );
}

export default ManagerBar;

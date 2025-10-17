import styles from './ManagerBar.module.css'
import Add from "../Add/Add.jsx";


function ManagerBar() {
    return(
        <div className={styles.bar}>
            <Add> </Add>
        </div>
    );
}

export default ManagerBar;

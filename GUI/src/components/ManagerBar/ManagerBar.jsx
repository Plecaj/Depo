import styles from './ManagerBar.module.css'
import Add from "../Add/Add.jsx";
import Install from "../Install/Install.jsx";
import Build from "../Build/Build.jsx";


function ManagerBar() {
    return(
        <div className={styles.bar}>
            <Add/>
            <Install/>
            <Build/>
        </div>
    );
}

export default ManagerBar;

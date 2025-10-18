import styles from'./HelpBar.module.css';
import icon from '../../assets/icon.png'
import SelectProject from '../SelectProject/SelectProject.jsx';
import Init from '../Init/Init.jsx';

function HelpBar() {

    return(
        <>
            <div className={styles.bar}>
                <img src={icon} alt="icon" className={styles.icon}></img>
                <div className={styles.line}>.</div>
                <SelectProject/>
                <Init/>
            </div>
        </>
    );
}

export default HelpBar;


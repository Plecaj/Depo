import styles from'./HelpBar.module.css';
import icon from '../../assets/icon.png'

function HelpBar() {

    return(
        <>
            <div className={styles.bar}>
                <img src={icon} alt="icon" className={styles.icon}></img>
                <div className={styles.line}>.</div>
            </div>
        </>
    );
}

export default HelpBar;


import styles from './SettingsWindow.module.css';
import closeIcon from "../../assets/delete.png";


function AddWindow({isSettingsVisible, setIsSettingsVisible}) {

    return(
        <>
            {isSettingsVisible &&
                <div  className={styles.backGround}>
                    <div className={styles.window}>

                        <div className={styles.header}>
                            <button className={styles.closeButton} onClick={() => setIsSettingsVisible(false)}> <img src={closeIcon} alt="X"></img> </button>
                        </div>


                    </div>
                </div>
            }
        </>
    );
}

export default AddWindow;

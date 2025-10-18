import styles from'./SelectProject.module.css';
import { open } from "@tauri-apps/plugin-dialog";

function SelectProject() {

    async function select() {
        const okok = await open({
            multiple: false,
            directory: true,
        });
       console.log(okok);
    }

    return(
        <>
            <div className={styles.button} onClick={select} > Select Project </div>
        </>
    );
}

export default SelectProject;


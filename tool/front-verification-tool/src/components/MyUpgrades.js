import React, { useState, useEffect, useRef } from 'react';
import { DataTable } from 'primereact/datatable';
import { Column } from 'primereact/column';
import { Button } from 'primereact/button';
import API from '../service/API';
import Backdrop from '@material-ui/core/Backdrop';
import CircularProgress from '@material-ui/core/CircularProgress';
import { makeStyles } from '@material-ui/core/styles';
import { Toast } from 'primereact/toast';

const useStyles = makeStyles((theme) => ({
    backdrop: {
      zIndex: theme.zIndex.drawer + 1,
      color: '#fff',
    },
  }));

const TableDemo = () => {
    const [logs, setLogs] = useState([]);
    const [account, setAccount] = useState('');
    const [loading, setLoading] = useState(false);
    const classes=useStyles();
    const toast = useRef();
    
    useEffect(() => { list_upgrades(); }, []);

    const formatDate = (value) => {
        if(value.created_at ){
            let date = value.created_at.split("T");
            let day_month_year = date[0].split("-");
            let hour_minute = date[1].split(":");
            return `${day_month_year[2]}/${day_month_year[1]}/${day_month_year[0]} ${hour_minute[0]}:${hour_minute[1]}`
        }
    }

    function show_toast(severity, summary) {
        toast.current.show({ severity: severity , summary: summary, detail: '', life: 6000 });
    }

    function list_upgrades(){
        try {
        
            if(window.ethereum == undefined){
                show_toast('error', 'Your wallet is not connected');
                return;
            }
            setLoading(true);
            API.get(`/listupgrades/${window.ethereum.selectedAddress}`).then(async res => {
                setLogs(res.data);
                setLoading(false);
            }).catch(error => {
                show_toast('error', 'Your upgrades could not be fetched');
                setLoading(false);
            });
        } catch (error){
            show_toast('error', 'There was an unexpected error');
            setLoading(false);
        }
    }

    return (
        <div className="grid table-demo">

        {loading    ?   <Backdrop className={classes.backdrop} open>
                            <CircularProgress color="inherit" />
                        </Backdrop> : loading}

        <div className="col-12">
        <Toast ref={toast} />    
            <div className="card">
            <h5>My Upgrades</h5>
            <DataTable value={logs} rows={5} paginator responsiveLayout="scroll">
                
                <Column field="specification_id" header="Specification ID" sortable style={{width: '20%'}}/>

                <Column field="proxy_address" header="Proxy Address" sortable style={{width: '35%'}}/>

                <Column field="created_at"  body={formatDate} header="Created At" sortable style={{width: '35%'}}/>
                        
                <Column header="Details" style={{width:'15%'}} body={() => (
                    <>
                        <Button icon="pi pi-search" type="button" className="p-button-text"/>
                    </>
                )}/>
            </DataTable>
            </div>
        </div>   
           
        </div>
    );
}

const comparisonFn = function (prevProps, nextProps) {
    return prevProps.location.pathname === nextProps.location.pathname;
};

export default React.memo(TableDemo, comparisonFn);
